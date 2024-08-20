type NSize = u8;
type AnsSize = u16;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Package {
    using: [bool; 3],
    flush_time: [NSize; 3],
    flush_duration: [NSize; 3],
    duration_used: Vec<bool>,
}

struct State {
    time: AnsSize,
    package: Package,
}

#[derive(Debug, Clone, Copy)]
enum Toilet {
    Left,
    Middle,
    Right,
}

impl Package {
    fn is_finish(&self) -> bool {
        self.using.iter().all(|x| *x == false) && self.duration_used.iter().all(|x| *x)
    }
}

fn skip_time(state: &State) -> State {
    let mut package = state.package.clone();
    let Package {
        flush_time, using, flush_duration, ..
    } = &mut package;

    let active_toilets: Vec<Toilet> = [Toilet::Left, Toilet::Middle, Toilet::Right]
        .into_iter()
        .filter(|toilet| flush_time[*toilet as usize] > 0)
        .collect();

    let next_flush = active_toilets
        .iter()
        .map(|toilet| flush_time[*toilet as usize])
        .min()
        .unwrap_or(0);

    for toilet in active_toilets {
        let time = &mut flush_time[toilet as usize];
        *time -= next_flush;

        if *time == 0 && using[toilet as usize] {
            using[toilet as usize] = false;
            flush_time[toilet as usize] = flush_duration[toilet as usize];
        };
    }

    State {
        time: state.time + next_flush as AnsSize,
        package,
    }
}

fn use_toilet(state: &State, toilet: Toilet, duration: NSize) -> State {
    let mut package = state.package.clone();

    package.flush_time[toilet as usize] = duration;
    package.using[toilet as usize] = true;
    package.flush_duration[toilet as usize] = duration;
    package.duration_used[duration as usize] = true;

    State {
        time: state.time as AnsSize,
        package,
    }
}

#[derive(Debug)]
struct ToiletProblem {
    n: NSize,
    best_answer: AnsSize,
}

impl ToiletProblem {
    fn new(n: NSize) -> ToiletProblem {
        let mut problem = ToiletProblem {
            n,
            best_answer: (n as AnsSize) * (n as AnsSize + 1),
        };

        let mut duration_used = vec![false; n as usize + 1];
        duration_used[0] = true;

        let start = State {
            time: 0,
            package: Package {
                using: [false; 3],
                flush_time: [0; 3],
                flush_duration: [0; 3],
                duration_used,
            },
        };

        // let mut memo : HashMap<State, _> = HashMap::new();

        problem.bf(start);

        problem
    }

    fn bf(&mut self, cur: State) {
        if cur.time > self.best_answer {
            return;
        }

        if cur.package.is_finish() {
            self.best_answer = cur.time;
            return;
        }

        // Try skipping time.
        {
            let new_state = skip_time(&cur);
            if new_state.time > cur.time {
                self.bf(new_state)
            }
        }

        let Package {
            using,
            duration_used,
            flush_time,
            ..
        } = &cur.package;

        for i in 1..=self.n {
            if duration_used[i as usize] {
                continue;
            }

            for toilet in [Toilet::Left, Toilet::Right] {
                if !using[Toilet::Middle as usize] && flush_time[toilet as usize] == 0 {
                    self.bf(use_toilet(&cur, toilet, i));
                }
            }

            if !using[Toilet::Left as usize]
                && !using[Toilet::Right as usize]
                && flush_time[Toilet::Middle as usize] == 0
            {
                self.bf(use_toilet(&cur, Toilet::Middle, i));
            }
        }
    }
}

fn main() {
    for n in 1..=10 {
        let p = ToiletProblem::new(n);
        dbg!(p);
    }
}
