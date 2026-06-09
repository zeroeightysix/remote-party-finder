use crate::listing::PartyFinderListing;
use crate::listing_container::{ListingContainer, QueriedListing};
use anyhow::Context;
use chrono::{TimeDelta, Utc};
use futures_util::StreamExt;
use mongodb::bson::doc;
use mongodb::options::UpdateOptions;
use mongodb::results::UpdateResult;
use mongodb::Collection;

pub async fn get_current_listings(
    collection: Collection<ListingContainer>,
) -> anyhow::Result<Vec<QueriedListing>> {
    let two_hours_ago = Utc::now() - TimeDelta::try_hours(2).unwrap();
    let cursor = collection
        .aggregate(
            [
                // don't ask me why, but mongo shits itself unless you provide a hard date
                // doc! {
                //     "$match": {
                //         "created_at": {
                //             "$gte": {
                //                 "$dateSubtract": {
                //                     "startDate": "$$NOW",
                //                     "unit": "hour",
                //                     "amount": 2,
                //                 },
                //             },
                //         },
                //     }
                // },
                doc! {
                    "$match": {
                        "updated_at": { "$gte": two_hours_ago },
                    }
                },
                doc! {
                    "$match": {
                        // filter private pfs
                        "listing.search_area": { "$bitsAllClear": 2 },
                    }
                },
                doc! {
                    "$set": {
                        "time_left": {
                            "$divide": [
                                {
                                    "$subtract": [
                                        { "$multiply": ["$listing.seconds_remaining", 1000] },
                                        { "$subtract": ["$$NOW", "$updated_at"] },
                                    ]
                                },
                                1000,
                            ]
                        },
                        "updated_minute": {
                            "$dateTrunc": {
                                "date": "$updated_at",
                                "unit": "minute",
                                "binSize": 5,
                            },
                        },
                    }
                },
                doc! {
                    "$match": {
                        "time_left": { "$gte": 0 },
                    }
                },
            ],
            None,
        )
        .await?;

    let collect = cursor
        .filter_map(async |res| {
            res.ok()
                .and_then(|doc| mongodb::bson::from_document(doc).ok())
        })
        .collect::<Vec<_>>()
        .await;

    Ok(collect)
}

pub async fn insert_listing(
    collection: Collection<ListingContainer>,
    listing: &PartyFinderListing,
) -> anyhow::Result<UpdateResult> {
    if listing.created_world >= 1_000
        || listing.home_world >= 1_000
        || listing.current_world >= 1_000
    {
        anyhow::bail!("invalid listing");
    }

    let opts = UpdateOptions::builder().upsert(true).build();
    let bson_value = mongodb::bson::to_bson(&listing)?;
    let now = Utc::now();
    collection
        .update_one(
            doc! {
                "listing.id": listing.id,
                "listing.last_server_restart": listing.last_server_restart,
                "listing.created_world": listing.created_world as u32,
            },
            doc! {
                "$currentDate": {
                    "updated_at": true,
                },
                "$set": {
                    "listing": bson_value,
                },
                "$setOnInsert": {
                    "created_at": now,
                },
            },
            opts,
        )
        .await
        .context("could not insert record")
}
