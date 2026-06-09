use chrono::{DateTime, Utc};
use serde::Serialize;
use sestring::SeString;

use crate::{
    ffxiv::{self, Language},
    listing::{
        ConditionFlags, DutyFinderSettingsFlags, LootRuleFlags, ObjectiveFlags, PartyFinderListing,
        PartyFinderSlot, SearchAreaFlags,
    },
    listing_container::QueriedListing,
    sestring_ext::SeStringExt,
};

/// A version of `QueriedListingContainer` with more sensible formatting,
/// implementation details hidden, and resolved names for duties, etc.
#[derive(Serialize)]
pub struct ApiReadableListingContainer {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    time_left: f64,
    listing: ApiReadableListing,
}

impl From<QueriedListing> for ApiReadableListingContainer {
    fn from(value: QueriedListing) -> Self {
        Self {
            created_at: value.created_at,
            updated_at: value.updated_at,
            time_left: value.time_left,
            listing: value.listing.into(),
        }
    }
}

#[derive(Serialize)]
pub struct ApiReadableListing {
    id: u32,
    // pub content_id: u32,
    recruiter: String,
    description: ApiLocalizedString,
    created_world: ApiReadableWorld,
    home_world: ApiReadableWorld,
    current_world: ApiReadableWorld,
    // `Debug` of `DutyCategory`
    category: String,
    duty_info: Option<ApiReadableDutyInfo>,
    // `Debug` of `DutyType`
    duty_type: String,
    beginners_welcome: bool,
    seconds_remaining: u16,
    min_item_level: u16,
    num_parties: u8,
    slot_count: u8, // = slots_available
    last_server_restart: u32,
    objective: ApiReadableObjectiveFlags,
    conditions: ApiReadableConditionFlags,
    duty_finder_settings: ApiReadableDutyFinderSettingsFlags,
    loot_rules: ApiReadableLootRuleFlags,
    search_area: ApiReadableSearchAreaFlags,
    slots: Vec<ApiReadablePartyFinderSlot>,
    slots_filled: Vec<Option<&'static str>>, // None if not filled, otherwise the job code
}

impl From<PartyFinderListing> for ApiReadableListing {
    fn from(value: PartyFinderListing) -> Self {
        let duty_info = ffxiv::duty(value.duty as u32).map(|di| di.into());
        let slots_filled = value
            .jobs_present
            .into_iter()
            .map(|job| {
                if job == 0 {
                    None
                } else {
                    ffxiv::jobs::JOBS.get(&(job as u32)).map(|j| j.code())
                }
            })
            .collect();

        Self {
            id: value.id,
            recruiter: value.name.text(),
            description: value.description.into(),
            created_world: value.created_world.into(),
            home_world: value.home_world.into(),
            current_world: value.current_world.into(),
            category: format!("{:?}", value.category),
            duty_info,
            duty_type: format!("{:?}", value.duty_type),
            beginners_welcome: value.beginners_welcome,
            seconds_remaining: value.seconds_remaining,
            min_item_level: value.min_item_level,
            num_parties: value.num_parties,
            slot_count: value.slots_available,
            last_server_restart: value.last_server_restart,
            objective: value.objective.into(),
            conditions: value.conditions.into(),
            duty_finder_settings: value.duty_finder_settings.into(),
            loot_rules: value.loot_rules.into(),
            search_area: value.search_area.into(),
            slots: value.slots.into_iter().map(|s| s.into()).collect(),
            slots_filled,
        }
    }
}

#[derive(Serialize)]
pub struct ApiReadableWorld {
    id: u16,
    name: &'static str,
}

impl From<u16> for ApiReadableWorld {
    fn from(value: u16) -> Self {
        Self {
            id: value,
            name: crate::ffxiv::WORLDS
                .get(&(value as u32))
                .map(|w| w.as_str())
                .unwrap_or("Unknown"),
        }
    }
}

#[derive(Serialize)]
pub struct ApiReadableDutyInfo {
    pub name: ffxiv::LocalisedText,
    pub high_end: bool,
    pub content_kind_id: u32,
    pub content_kind: String,
}

impl From<&ffxiv::duties::DutyInfo> for ApiReadableDutyInfo {
    fn from(value: &ffxiv::duties::DutyInfo) -> Self {
        Self {
            name: value.name,
            high_end: value.high_end,
            content_kind_id: value.content_kind.as_u32(),
            content_kind: format!("{:?}", value.content_kind),
        }
    }
}

#[derive(Serialize)]
pub struct ApiReadableObjectiveFlags {
    duty_completion: bool,
    practice: bool,
    loot: bool,
}

impl From<ObjectiveFlags> for ApiReadableObjectiveFlags {
    fn from(value: ObjectiveFlags) -> Self {
        Self {
            duty_completion: value.contains(ObjectiveFlags::DUTY_COMPLETION),
            practice: value.contains(ObjectiveFlags::PRACTICE),
            loot: value.contains(ObjectiveFlags::LOOT),
        }
    }
}

#[derive(Serialize)]
struct ApiReadableConditionFlags {
    duty_complete: bool,
    duty_incomplete: bool,
    duty_complete_reward_unclaimed: bool,
}

impl From<ConditionFlags> for ApiReadableConditionFlags {
    fn from(value: ConditionFlags) -> Self {
        Self {
            duty_complete: value.contains(ConditionFlags::DUTY_COMPLETE),
            duty_incomplete: value.contains(ConditionFlags::DUTY_INCOMPLETE),
            duty_complete_reward_unclaimed: value
                .contains(ConditionFlags::DUTY_COMPLETE_WEEKLY_REWARD_UNCLAIMED),
        }
    }
}

#[derive(Serialize)]
struct ApiReadableDutyFinderSettingsFlags {
    undersized_party: bool,
    minimum_item_level: bool,
    silence_echo: bool,
}

impl From<DutyFinderSettingsFlags> for ApiReadableDutyFinderSettingsFlags {
    fn from(value: DutyFinderSettingsFlags) -> Self {
        Self {
            undersized_party: value.contains(DutyFinderSettingsFlags::UNDERSIZED_PARTY),
            minimum_item_level: value.contains(DutyFinderSettingsFlags::MINIMUM_ITEM_LEVEL),
            silence_echo: value.contains(DutyFinderSettingsFlags::SILENCE_ECHO),
        }
    }
}

#[derive(Serialize)]
struct ApiReadableLootRuleFlags {
    greed_only: bool,
    lootmaster: bool,
}

impl From<LootRuleFlags> for ApiReadableLootRuleFlags {
    fn from(value: LootRuleFlags) -> Self {
        Self {
            greed_only: value.contains(LootRuleFlags::GREED_ONLY),
            lootmaster: value.contains(LootRuleFlags::LOOTMASTER),
        }
    }
}

#[derive(Serialize)]
struct ApiReadableSearchAreaFlags {
    data_centre: bool,
    private: bool,
    alliance_raid: bool,
    world: bool,
    one_player_per_job: bool,
}

impl From<SearchAreaFlags> for ApiReadableSearchAreaFlags {
    fn from(value: SearchAreaFlags) -> Self {
        Self {
            data_centre: value.contains(SearchAreaFlags::DATA_CENTRE),
            private: value.contains(SearchAreaFlags::PRIVATE),
            alliance_raid: value.contains(SearchAreaFlags::ALLIANCE_RAID),
            world: value.contains(SearchAreaFlags::WORLD),
            one_player_per_job: value.contains(SearchAreaFlags::ONE_PLAYER_PER_JOB),
        }
    }
}

#[derive(Serialize)]
pub struct ApiReadablePartyFinderSlot(Vec<&'static str>); // list of job codes

impl From<PartyFinderSlot> for ApiReadablePartyFinderSlot {
    fn from(value: PartyFinderSlot) -> Self {
        Self(
            value
                .accepting
                .classjobs()
                .into_iter()
                .map(|cj| cj.code())
                .collect(),
        )
    }
}

#[derive(Serialize)]
pub struct ApiLocalizedString {
    en: String,
    ja: String,
    de: String,
    fr: String,
}

impl From<SeString> for ApiLocalizedString {
    fn from(value: SeString) -> Self {
        Self {
            en: value.full_text(&Language::English),
            ja: value.full_text(&Language::Japanese),
            de: value.full_text(&Language::German),
            fr: value.full_text(&Language::French),
        }
    }
}
