use std::collections::HashSet;

pub struct Combinatoric<T: Clone> {
    items: Vec<T>,
    take: usize,
    indices: Vec<usize>,
    exhausted: bool,
}

impl<T: Clone> Combinatoric<T> {
    pub fn new(items: Vec<T>, take: usize) -> Self {
        assert!(
            items.len() >= take,
            "population must be at least as large as take"
        );

        let mut indices = Vec::new();
        for i in 0..take {
            indices.push(i);
        }

        Self {
            items,
            take,
            indices,
            exhausted: false,
        }
    }

    fn advance(&mut self) -> bool {
        let mut member = self.take - 1;
        loop {
            let max_index = self.items.len() - 1 - (self.take - member - 1);

            if self.indices[member] < max_index {
                self.indices[member] += 1;
                for i in (member + 1)..self.take {
                    self.indices[i] = self.indices[i - 1] + 1;
                }
                return true;
            } else if member > 0 {
                member -= 1;
            } else {
                return false;
            }
        }
    }

    /// Build the actual groups from current index state
    fn build_current(&self) -> Vec<T> {
        self.indices.iter().map(|index| self.items[*index].clone()).collect()
    }
}

impl<T: Clone> Iterator for Combinatoric<T> {
    type Item = Vec<T>; // <- Note: probably want Vec<Vec<T>> not Vec<T>

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }

        let result = self.build_current();

        if !self.advance() {
            self.exhausted = true;
        }

        Some(result)
    }
}

pub struct GroupsCombinatoric<T: Clone> {
    items: Vec<T>,
    take_order: Vec<usize>,
    sub_combinatorics: Vec<Combinatoric<T>>,
    exhausted: bool,
}

impl<T: Clone> GroupsCombinatoric<T> {
    pub fn new(items: Vec<T>, take_order: Vec<usize>) -> Self {
        assert!(
            items.len() >= take_order.iter().sum::<usize>(),
            "population must be at least as large as take_order.iter().sum::<usize>()"
        );

        assert!(take_order.len() > 0, "take_order must contain at least 1 group");

        let base_combinatoric = Combinatoric::new(items.clone(), take_order[0]);
        let mut sub_combinatorics = Vec::from([base_combinatoric]);
        let mut untouchable_indices = sub_combinatorics[0].indices.iter().map(|index| *index).collect::<HashSet<usize>>();
        for i in 1..take_order.len() {
          let subset = items.iter().enumerate().filter(|(index, _)| !untouchable_indices.contains(index)).map(|(_, item)| item.clone()).collect();
          sub_combinatorics.push(Combinatoric::new(subset, take_order[i]));
          untouchable_indices.extend(sub_combinatorics[i].indices.iter().map(|index| *index));
        }

        Self {
          items,
          take_order,
          sub_combinatorics,
          exhausted: false,
        }
    }

    fn build_current(&self) -> Vec<Vec<T>> {
        self.sub_combinatorics.iter().map(|combinatoric| combinatoric.build_current()).collect()
    }

    fn advance(&mut self) -> bool {
        // If any of the sub_combinatorics are exhausted, starting with the deepest (rightmost in the vec), advance the one directly left of it, reinitialize ALL sub_combinatorics to the right of the advanced one, then return true. When the leftmost sub_combinatoric is exhausted, we know we've explored everything.
        for i in (0..self.sub_combinatorics.len()).rev() {
          let can_advance = self.sub_combinatorics[i].advance();
          if !can_advance && i == 0 {
            self.exhausted = true;
            return false;
          } else if can_advance {
            let mut untouchable_indices = self.sub_combinatorics[..=i].iter().map(|combinatoric| combinatoric.indices.iter().map(|index| *index).collect::<HashSet<usize>>()).flatten().collect::<HashSet<usize>>();
            for j in (i + 1)..self.sub_combinatorics.len() {
              self.sub_combinatorics[j] = Combinatoric::new(self.items.iter().enumerate().filter(|(index, _)| !untouchable_indices.contains(index)).map(|(_, item)| item.clone()).collect(), self.take_order[j]);
              untouchable_indices.extend(self.sub_combinatorics[j].indices.iter().map(|index| *index));
            }
            return true;
          }
        }
        return false;
    }

    pub fn next(&mut self) -> Option<Vec<Vec<T>>> {
        if self.exhausted {
            return None;
        }

        let result = self.build_current();

        if !self.advance() {
            self.exhausted = true;
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinatoric() {
        let mut combinatoric = Combinatoric::new(vec![1, 2, 3, 4], 2);
        let mut num_results = 0;
        while let Some(result) = combinatoric.next() {
            println!("{:?}", result);
            num_results += 1;
        }
        assert_eq!(num_results, 6);
    }

    #[test]
    fn test_groups_combinatoric() {
        let mut combinatoric = GroupsCombinatoric::new(vec![1, 2, 3, 4, 5], vec![3, 1]);
        let mut num_results = 0;
        while let Some(result) = combinatoric.next() {
            println!("{:?}", result);
            num_results += 1;
        }
        assert_eq!(num_results, 20);
    }

    #[test]
    fn test_groups_combinatoric_large() {
      // Make a group of 100
      let group = (0..20).collect::<Vec<usize>>();
      let mut combinatoric = GroupsCombinatoric::new(group, vec![2, 2, 2, 2]);
      let mut num_results = 0;
      while let Some(result) = combinatoric.next() {
        if num_results % 10000 == 0 {
          println!("{:?}", result);
          // println!("{}", num_results);
        }
        num_results += 1;
      }
      assert_eq!(num_results, 1000000);
    }
}