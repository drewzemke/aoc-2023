pub mod parser;
pub mod puzzle12a;
pub mod puzzle12b;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SpringState {
    Damaged,
    Operational,
    Unknown,
}

impl SpringState {
    pub fn could_be_damaged(&self) -> bool {
        *self == SpringState::Damaged || *self == SpringState::Unknown
    }

    pub fn could_be_operational(&self) -> bool {
        *self == SpringState::Operational || *self == SpringState::Unknown
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SpringRow(Vec<SpringState>);

#[derive(Debug, PartialEq, Eq)]
pub struct DamagedGroups(Vec<usize>);

#[derive(Debug, PartialEq, Eq)]
pub struct Schematic(SpringRow, DamagedGroups);

impl Schematic {
    pub fn count_arrangements(&self) -> u32 {
        let Schematic(SpringRow(springs), DamagedGroups(groups)) = self;

        // if there are no damaged groups, and all of the springs are not explicitly damaged,
        // it means we successfully matched all of them, so return 1
        if groups.is_empty() {
            if springs.iter().all(SpringState::could_be_operational) {
                return 1;
            } else {
                return 0;
            }
        }

        // otherwise pick off the first damaged group
        let first_group = groups.first().unwrap();

        // if there aren't enough springs left to match this group, just bail
        if springs.len() < *first_group {
            return 0;
        }

        let mut arrangements = 0;

        // walk the row until we find `first_group` consecutive damaged/unknown springs
        for idx in 0..springs.len() - first_group + 1 {
            if springs[idx..idx + first_group]
                .iter()
                .all(SpringState::could_be_damaged)
            {
                // now check the springs before and after this chunk -- if they exist,
                // make sure they _could_ be operational
                // (that is, make sure we've matched on a whole damaged group, rather than a subset of one)
                let prev_spring_op = idx == 0 || springs[idx - 1].could_be_operational();
                let next_spring_op = springs.len() == idx + *first_group
                    || springs[idx + first_group].could_be_operational();

                if prev_spring_op && next_spring_op {
                    // chop off the bits we've already processed and recurse
                    let next_spring_slice_start = springs.len().min(idx + first_group + 1);
                    arrangements += Schematic(
                        SpringRow(springs[next_spring_slice_start..].into()),
                        DamagedGroups(groups[1..].into()),
                    )
                    .count_arrangements();
                }
            }

            // if the first spring of the current window is damaged, we have to match on it,
            // so we shouldn't check the rest of the windows
            if springs[idx] == SpringState::Damaged {
                break;
            }
        }

        arrangements
    }
}
