use crate::phrases::Phrases;
use crate::world::*;
use std::fs;
use std::path::Path;

/// Builder for the game's `World`.
pub struct Builder {
    world: World,
}

impl Builder {
    const DEFAULT_PHRASES_FILE_NAME: &'static str = "default";

    pub fn new() -> Self {
        Builder {
            world: World::new(),
        }
    }

    /// Deserialize players and cooks into the world.
    pub fn with_players_file(mut self, path: impl AsRef<Path>) -> Self {
        let contents = fs::read_to_string(path).expect("Could not open game config file.");
        let (cooks, judges): (Vec<Cook>, Vec<Judge>) =
            ron::de::from_str(&contents).expect("Could not parse players file.");

        self.world.cooks = cooks;
        self.world.cooks_in_game = (0..self.world.cooks.len()).into_iter().collect();
        self.world.judges = judges;

        self
    }

    // TODO
    pub fn with_phrases_dir(mut self, path: impl AsRef<Path>) -> std::io::Result<Self> {
        for entry in fs::read_dir(path)? {
            let phrases_file = entry?;

            // No subdirectories are allowed for now.
            if !phrases_file.file_type().unwrap().is_file() {
                panic!(
                    "The phrases directory at {:?} should not contain any subdirectories...",
                    phrases_file.path()
                );
            }

            // The name of the file corresponds to its judge's lowercase name. Here each phrases
            // file is associated to its judge.
            let judge_id = phrases_file
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();

            // To avoid repetition.
            let parse_phrases_file =
                || Phrases::parse(&fs::read_to_string(phrases_file.path()).unwrap());

            // The file which is identified as the default phrases file is handled differently.
            if judge_id == Self::DEFAULT_PHRASES_FILE_NAME {
                self.world.default_phrases = parse_phrases_file();

                continue;
            }

            let mut judge = self
                .world
                .judges
                .iter_mut()
                .find(|j| j.name.to_lowercase() == judge_id)
                .unwrap_or_else(|| {
                    panic!(
                        "Could not find judge named {} while parsing phrases file {:?}",
                        judge_id,
                        phrases_file.path()
                    )
                });
            judge.phrases = parse_phrases_file();
        }

        Ok(self)
    }

    /// Return the built world.
    pub fn build(self) -> World {
        self.world
    }
}
