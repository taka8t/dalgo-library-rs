
pub struct SatSolver {
    n: usize,
    pos: Vec<usize>,
    neg: Vec<usize>,
    remcnt: Vec<usize>,
    clauses: Vec<Vec<usize>>,
    decision_stack: Vec<Vec<usize>>,
    literal_pos: Vec<Vec<usize>>,
    assigns: Vec<bool>,
    watch_clause_list: Vec<usize>,
}

impl SatSolver {
    pub fn new(n: usize) -> Self {
        Self {
            n: n,
            pos: Vec::new(),
            neg: Vec::new(),
            remcnt: vec![0; n*2],
            clauses: Vec::new(),
            decision_stack: vec![Vec::new(); 1],
            literal_pos: vec![Vec::new(); n*2],
            assigns: vec![false; n*2],
            watch_clause_list: Vec::new(),
        }
    }

    pub fn add_clause(&mut self, cl: Vec<usize>) {
        let id = self.clauses.len();
        for i in cl.iter().copied() {
            self.literal_pos[i + self.n].push(id);
            self.remcnt[i + self.n] += 1;
        }
        self.clauses.push(cl);
    }

    pub fn push(&mut self, lit_id: usize) {
        let id = lit_id + self.n;
        self.assigns[id] = true;
        self.decision_stack.last_mut().unwrap().push(lit_id);
        for &i in self.literal_pos[id].iter() {
            if self.pos[i] == 0 {
                for &j in self.clauses[i].iter() {
                    self.remcnt[j + self.n] -= 1;
                }
            }
            self.pos[i] += 1;
        }
        let id = !lit_id + self.n;
        for &i in self.literal_pos[id].iter() {
            if self.pos[i] == 0 {
                self.watch_clause_list.push(i);
            }
            self.neg[i] += 1;
        }
    }

    pub fn pop(&mut self) {
        let lit_id = self.decision_stack.last_mut().unwrap().pop().unwrap();
        let id = lit_id + self.n;
        self.assigns[id] = false;
        for &i in self.literal_pos[id].iter() {
            self.pos[i] -= 1;
            if self.pos[i] == 0 {
                for &j in self.clauses[i].iter() {
                    self.remcnt[j + self.n] += 1;
                }
            }
        }
        let id = !lit_id + self.n;
        for &i in self.literal_pos[id].iter() {
            self.neg[i] -= 1;
        }
    }

    pub fn propagate(&mut self) -> bool {
        while let Some(lit_id) = self.watch_clause_list.pop() {
            if self.pos[lit_id] > 0 {continue;}
            if self.neg[lit_id] == self.clauses[lit_id].len() {return false;}
            if self.neg[lit_id] + 1 == self.clauses[lit_id].len() {
                let nx = if let Some(&nx) = self.clauses[lit_id].iter().filter(|&x| !self.assigns[*x+self.n] && !self.assigns[!*x+self.n]).next() {
                    nx
                }
                else {self.n};
                if self.assigns[!nx+self.n] {return false;}
                self.push(nx);
            }
        }
        true
    }

    pub fn solve(&mut self) -> bool {
        self.pos = vec![0; self.clauses.len()];
        self.neg = vec![0; self.clauses.len()];

        loop {
            if self.propagate() {
                if let Some(nid) = (0..self.n)
                .filter(|&i| self.remcnt[i+self.n] + self.remcnt[!i+self.n] > 0)
                .max_by(|&a, &b| (self.remcnt[a+self.n] + self.remcnt[!a+self.n]).cmp(&(self.remcnt[b+self.n] + self.remcnt[!b+self.n]))) {
                    self.decision_stack.push(Vec::new());
                    self.push(nid);
                }
                else {return true;}
            }
            else {
                let nid = self.decision_stack.last().unwrap()[0];
                while !self.decision_stack.last().unwrap().is_empty() {self.pop();}
                self.decision_stack.pop();
                if self.decision_stack.is_empty() {return false;}
                self.push(!nid);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::path::Path;
    
    #[test]
    fn sat_test(){
        
        for case_id in 1..21 {
            let file_string = format!("./src/algorithm/data/jnh{}.cnf", case_id);
            let test_file = Path::new(&file_string);
            let mut test_data = String::new();
            let mut f = File::open(&test_file).unwrap();
            f.read_to_string(&mut test_data).unwrap();
            let mut input = test_data.split_whitespace();

            loop {
                let s = read!(input, String);
                if s.as_str() == "cnf" {break;}
            }
            input!{
                input,
                n: usize,
                m: usize,
            }
            let mut solver = SatSolver::new(n);
            for _ in 0..m {
                let mut tcl = Vec::new();
                loop {
                    let mut pi = read!(input, i32);
                    if pi == 0 {break;}
                    if pi > 0 {pi -= 1;}
                    tcl.push(pi as usize);
                }
                solver.add_clause(tcl);
            }
            if solver.solve() {
                print!("{} : SAT\n", case_id);
            }
            else {
                print!("{} : UNSAT\n", case_id);
            }
        }
    }
}