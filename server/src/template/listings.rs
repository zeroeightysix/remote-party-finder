use crate::ffxiv::Language;
use crate::listing::JobFlags;
use crate::listing::PartyFinderCategory;
use crate::listing_container::QueriedListing;
use crate::sestring_ext::SeStringExt;
use askama::Template;
use std::borrow::Borrow;

#[derive(Debug, Template)]
#[template(path = "listings.html")]
pub struct ListingsTemplate {
    pub containers: Vec<QueriedListing>,
    pub lang: Language,
}
