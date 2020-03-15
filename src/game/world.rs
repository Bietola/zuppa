/// Cook entity.
pub struct Cook {
    name: String,
}

type CookKey = usize;

/// Judge entity.
pub struct Judge {
    name: String,
}

type JudgeKey = usize;

/// Entire state of the game world.
pub struct GameWorld {
    cooks: Vec<Cook>,
    judge: Vec<Judge>,
}
