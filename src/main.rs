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
enum Urinal {
    Left,
    Middle,
    Right,
}

impl Package {
    fn is_finish(&self) -> bool {
        self.using.iter().all(|x| *x == false) && self.duration_used.iter().all(|x| *x)
    }
}

impl State {
    fn after_flush(&self) -> State {
        let mut package = self.package.clone();
        let Package {
            flush_time, using, flush_duration, ..
        } = &mut package;
    
        let active_urinals: Vec<Urinal> = [Urinal::Left, Urinal::Middle, Urinal::Right]
            .into_iter()
            .filter(|urinal| flush_time[*urinal as usize] > 0)
            .collect();
    
        let next_flush = active_urinals
            .iter()
            .map(|urinal| flush_time[*urinal as usize])
            .min()
            .unwrap_or(0);
    
        for urinal in active_urinals {
            let time = &mut flush_time[urinal as usize];
            *time -= next_flush;
    
            if *time == 0 && using[urinal as usize] {
                using[urinal as usize] = false;
                flush_time[urinal as usize] = flush_duration[urinal as usize];
            };
        }
    
        State {
            time: self.time + next_flush as AnsSize,
            package,
        }
    }

    fn use_urinal(&self, urinal: Urinal, duration: NSize) -> State {
        let mut package = self.package.clone();
    
        package.flush_time[urinal as usize] = duration;
        package.using[urinal as usize] = true;
        package.flush_duration[urinal as usize] = duration;
        package.duration_used[duration as usize] = true;
    
        State {
            time: self.time as AnsSize,
            package,
        }
    }
}

#[derive(Debug)]
struct UrinalProblem {
    n: NSize,
    best_answer: AnsSize,
}

impl UrinalProblem {
    fn new(n: NSize) -> UrinalProblem {
        let mut problem = UrinalProblem {
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
        if cur.time >= self.best_answer {
            return;
        }

        if cur.package.is_finish() {
            self.best_answer = cur.time;
            return;
        }

        // Try skipping time.
        {
            let new_state = cur.after_flush();
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

            for urinal in [Urinal::Left, Urinal::Right] {
                if !using[Urinal::Middle as usize] && flush_time[urinal as usize] == 0 {
                    self.bf(cur.use_urinal(urinal, i));
                }
            }

            if !using[Urinal::Left as usize]
                && !using[Urinal::Right as usize]
                && flush_time[Urinal::Middle as usize] == 0
            {
                self.bf(cur.use_urinal(Urinal::Middle, i));
            }
        }
    }
}

fn main() {
    for n in 1..=10 {
        let p = UrinalProblem::new(n);
        dbg!(p);
    }
}
