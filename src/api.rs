use crate::{
    ComputeParametersInput, FSRS, FSRSItem, FSRSReview, MemoryState, NextStates,
    compute_parameters as train_parameters, current_retrievability as retrievability,
};

#[derive(Debug, Clone, Copy)]
pub struct FsrsMemoryState {
    pub stability: f32,
    pub difficulty: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct FsrsReview {
    pub rating: u32,
    pub delta_t: u32,
}

#[derive(Debug, Clone)]
pub struct FsrsItemState {
    pub memory: FsrsMemoryState,
    pub interval: f32,
}

#[derive(Debug, Clone)]
pub struct FsrsNextStates {
    pub again: FsrsItemState,
    pub hard: FsrsItemState,
    pub good: FsrsItemState,
    pub easy: FsrsItemState,
}

pub fn default_parameters() -> Vec<f32> {
    crate::DEFAULT_PARAMETERS.to_vec()
}

pub fn fsrs5_default_decay() -> f32 {
    crate::FSRS5_DEFAULT_DECAY
}

pub fn fsrs6_default_decay() -> f32 {
    crate::FSRS6_DEFAULT_DECAY
}

pub fn next_states(
    current_memory_state: Option<FsrsMemoryState>,
    desired_retention: f32,
    days_elapsed: u32,
    parameters: Option<Vec<f32>>,
) -> Result<FsrsNextStates, String> {
    fsrs(parameters)?
        .next_states(
            current_memory_state.map(Into::into),
            desired_retention,
            days_elapsed,
        )
        .map(Into::into)
        .map_err(error_to_string)
}

pub fn memory_state(
    reviews: Vec<FsrsReview>,
    starting_state: Option<FsrsMemoryState>,
    parameters: Option<Vec<f32>>,
) -> Result<FsrsMemoryState, String> {
    fsrs(parameters)?
        .memory_state(
            FSRSItem {
                reviews: reviews.into_iter().map(Into::into).collect(),
            },
            starting_state.map(Into::into),
        )
        .map(Into::into)
        .map_err(error_to_string)
}

pub fn compute_parameters(items: Vec<Vec<FsrsReview>>) -> Result<Vec<f32>, String> {
    let train_set = items
        .into_iter()
        .map(|reviews| FSRSItem {
            reviews: reviews.into_iter().map(Into::into).collect(),
        })
        .collect();

    train_parameters(ComputeParametersInput {
        train_set,
        ..Default::default()
    })
    .map_err(error_to_string)
}

pub fn memory_state_from_sm2(
    ease_factor: f32,
    interval: f32,
    sm2_retention: f32,
    parameters: Option<Vec<f32>>,
) -> Result<FsrsMemoryState, String> {
    fsrs(parameters)?
        .memory_state_from_sm2(ease_factor, interval, sm2_retention)
        .map(Into::into)
        .map_err(error_to_string)
}

pub fn current_retrievability(state: FsrsMemoryState, days_elapsed: f32, decay: f32) -> f32 {
    retrievability(state.into(), days_elapsed, decay)
}

fn fsrs(parameters: Option<Vec<f32>>) -> Result<FSRS, String> {
    let parameters = parameters.unwrap_or_default();
    FSRS::new(&parameters).map_err(error_to_string)
}

fn error_to_string(error: crate::FSRSError) -> String {
    format!("{error:?}")
}

impl From<FsrsMemoryState> for MemoryState {
    fn from(value: FsrsMemoryState) -> Self {
        Self {
            stability: value.stability,
            difficulty: value.difficulty,
        }
    }
}

impl From<MemoryState> for FsrsMemoryState {
    fn from(value: MemoryState) -> Self {
        Self {
            stability: value.stability,
            difficulty: value.difficulty,
        }
    }
}

impl From<FsrsReview> for FSRSReview {
    fn from(value: FsrsReview) -> Self {
        Self {
            rating: value.rating,
            delta_t: value.delta_t,
        }
    }
}

impl From<NextStates> for FsrsNextStates {
    fn from(value: NextStates) -> Self {
        Self {
            again: value.again.into(),
            hard: value.hard.into(),
            good: value.good.into(),
            easy: value.easy.into(),
        }
    }
}

impl From<crate::ItemState> for FsrsItemState {
    fn from(value: crate::ItemState) -> Self {
        Self {
            memory: value.memory.into(),
            interval: value.interval,
        }
    }
}
