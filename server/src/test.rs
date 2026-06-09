use crate::listing::{
    ConditionFlags, DutyCategory, DutyFinderSettingsFlags, DutyType, JobFlags, LootRuleFlags,
    ObjectiveFlags, PartyFinderListing, PartyFinderSlot, SearchAreaFlags,
};
use sestring::SeString;

const LISTING: &str = r###"
{
  "id": 123,
  "content_id_lower": 456,
  "name": "VGVzdCBOYW1l",
  "description": "VGhpcyBpcyBteSB0ZXN0IGRlc2NyaXB0aW9uLg==",
  "created_world": 73,
  "home_world": 73,
  "current_world": 73,
  "category": 0,
  "duty": 55,
  "duty_type": 2,
  "beginners_welcome": false,
  "seconds_remaining": 3300,
  "min_item_level": 0,
  "num_parties": 1,
  "slots_available": 7,
  "last_server_restart": 0,
  "objective": 3,
  "conditions": 1,
  "duty_finder_settings": 0,
  "loot_rules": 0,
  "search_area": 1,
  "slots": [
    {
      "accepting": 167772160
    }
  ],
  "jobs_present": [
    5,
    0,
    0,
    0,
    0,
    0,
    0,
    0
  ]
}"###;

lazy_static::lazy_static! {
    static ref EXPECTED: PartyFinderListing = PartyFinderListing {
        id: 123,
        content_id_lower: 456,
        name: SeString::parse(b"Test Name").unwrap(),
        description: SeString::parse(b"This is my test description.").unwrap(),
        created_world: 73,
        home_world: 73,
        current_world: 73,
        category: DutyCategory::None,
        duty: 55,
        duty_type: DutyType::Normal,
        beginners_welcome: false,
        seconds_remaining: 3300,
        min_item_level: 0,
        num_parties: 1,
        slots_available: 7,
        last_server_restart: 0,
        objective: ObjectiveFlags::NONE | ObjectiveFlags::DUTY_COMPLETION,
        conditions: ConditionFlags::NONE,
        duty_finder_settings: DutyFinderSettingsFlags::NONE,
        loot_rules: LootRuleFlags::NONE,
        search_area: SearchAreaFlags::DATA_CENTRE,
        slots: vec![
            PartyFinderSlot {
                accepting: JobFlags::DANCER | JobFlags::BLUE_MAGE,
            },
        ],
        jobs_present: vec![5, 0, 0, 0, 0, 0, 0, 0],
    };
}

fn listing_with_objective(objective: ObjectiveFlags) -> PartyFinderListing {
    PartyFinderListing {
        id: 0,
        content_id_lower: 0,
        name: SeString::parse(b"").unwrap(),
        description: SeString::parse(b"").unwrap(),
        created_world: 0,
        home_world: 0,
        current_world: 0,
        category: DutyCategory::None,
        duty: 0,
        duty_type: DutyType::Other,
        beginners_welcome: false,
        seconds_remaining: 0,
        min_item_level: 0,
        num_parties: 0,
        slots_available: 0,
        last_server_restart: 0,
        objective,
        conditions: ConditionFlags::empty(),
        duty_finder_settings: DutyFinderSettingsFlags::NONE,
        loot_rules: LootRuleFlags::NONE,
        search_area: SearchAreaFlags::empty(),
        slots: vec![],
        jobs_present: vec![],
    }
}

#[test]
fn deserialise_listing() {
    let listing: PartyFinderListing = serde_json::from_str(LISTING).unwrap();
    assert_eq!(listing, *EXPECTED,)
}

#[test]
fn serialise_listing() {
    let json = serde_json::to_string_pretty(&*EXPECTED).unwrap();
    let round_trip: PartyFinderListing = serde_json::from_str(&json).unwrap();
    assert_eq!(round_trip, *EXPECTED);
}

#[test]
fn prepend_flags_practice() {
    let listing = listing_with_objective(ObjectiveFlags::PRACTICE);
    let (colour_class, flags) = listing.prepend_flags();
    assert_eq!(colour_class, "desc-green");
    assert_eq!(flags, "[Practice]");
}

#[test]
fn prepend_flags_duty_completion() {
    let listing = listing_with_objective(ObjectiveFlags::DUTY_COMPLETION);
    let (_, flags) = listing.prepend_flags();
    assert_eq!(flags, "[Duty Completion]");
}

#[test]
fn prepend_flags_loot() {
    let listing = listing_with_objective(ObjectiveFlags::LOOT);
    let (_, flags) = listing.prepend_flags();
    assert_eq!(flags, "[Loot]");
}

#[test]
fn prepend_flags_none_zero() {
    let listing = listing_with_objective(ObjectiveFlags::empty());
    let (_, flags) = listing.prepend_flags();
    assert!(flags.is_empty());
}

#[test]
fn prepend_flags_none_bit() {
    let listing = listing_with_objective(ObjectiveFlags::NONE);
    let (_, flags) = listing.prepend_flags();
    assert!(flags.is_empty());
}

#[test]
fn api_objective_mapping() {
    let practice = listing_with_objective(ObjectiveFlags::PRACTICE);
    assert!(!practice.objective.contains(ObjectiveFlags::DUTY_COMPLETION));
    assert!(practice.objective.contains(ObjectiveFlags::PRACTICE));
    assert!(!practice.objective.contains(ObjectiveFlags::LOOT));

    let loot = listing_with_objective(ObjectiveFlags::LOOT);
    assert!(!loot.objective.contains(ObjectiveFlags::DUTY_COMPLETION));
    assert!(!loot.objective.contains(ObjectiveFlags::PRACTICE));
    assert!(loot.objective.contains(ObjectiveFlags::LOOT));

    let duty_completion = listing_with_objective(ObjectiveFlags::DUTY_COMPLETION);
    assert!(duty_completion
        .objective
        .contains(ObjectiveFlags::DUTY_COMPLETION));
    assert!(!duty_completion.objective.contains(ObjectiveFlags::PRACTICE));
    assert!(!duty_completion.objective.contains(ObjectiveFlags::LOOT));
}
