use rltk::RandomNumberGenerator;

pub struct RandomEntry {
    name: String,
    weight: i32,
}

impl RandomEntry {
    pub fn new<S: ToString>(name: S, weight: i32) -> RandomEntry {
        RandomEntry {
            name: name.to_string(),
            weight,
        }
    }
}

#[derive(Default)]
pub struct RandomTable {
    entries: Vec<RandomEntry>,
    total_weight: i32,
}

impl RandomTable {
    pub fn new() -> RandomTable {
        RandomTable {
            entries: Vec::new(),
            total_weight: 0,
        }
    }

    /// Add item with weighted spawn chance to table.
    pub fn add<S: ToString>(mut self, name: S, weight: i32) -> RandomTable {
        // Items with weight less than 0 are ignored.
        if weight > 0 {
            self.total_weight += weight;
            self.entries
                .push(RandomEntry::new(name.to_string(), weight));
        }

        self
    }

    /// Roll dice and returns a random spawn table entry, with each item having equal chance proportional to its relative weight in the table.
    ///
    /// Rolls dice and iterates through table. Returns table entry name if the roll is less than the entry's weight. Otherwise, the roll is reduced by the weight and the next table entry is tested. Returns "None" if nothing is "selected" from the table.
    pub fn roll(&self, rng: &mut RandomNumberGenerator) -> String {
        if self.total_weight == 0 {
            return "None".to_string();
        }
        let mut roll = rng.roll_dice(1, self.total_weight) - 1;
        let mut index: usize = 0;

        while roll > 0 {
            if roll < self.entries[index].weight {
                return self.entries[index].name.clone();
            }

            roll -= self.entries[index].weight;
            index += 1;
        }

        "None".to_string()
    }
}
