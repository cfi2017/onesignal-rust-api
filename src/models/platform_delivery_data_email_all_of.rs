/*
 * OneSignal
 *
 * A powerful way to send personalized messages at scale and build effective customer engagement strategies. Learn more at onesignal.com
 *
 * The version of the OpenAPI document: 1.0.1
 * Contact: devrel@onesignal.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PlatformDeliveryDataEmailAllOf {
    /// Number of times an email has been opened.
    #[serde(rename = "opened", skip_serializing_if = "Option::is_none")]
    pub opened: Option<i32>,
    /// Number of unique recipients who have opened your email.
    #[serde(rename = "unique_opens", skip_serializing_if = "Option::is_none")]
    pub unique_opens: Option<i32>,
    /// Number of clicked links from your email. This can include the recipient clicking email links multiple times.
    #[serde(rename = "clicks", skip_serializing_if = "Option::is_none")]
    pub clicks: Option<i32>,
    /// Number of unique clicks that your recipients have made on links from your email.
    #[serde(rename = "unique_clicks", skip_serializing_if = "Option::is_none")]
    pub unique_clicks: Option<i32>,
    /// Number of recipients who registered as a hard or soft bounce and didn't receive your email.
    #[serde(rename = "bounced", skip_serializing_if = "Option::is_none")]
    pub bounced: Option<i32>,
    /// Number of recipients who reported this email as spam.
    #[serde(rename = "reported_spam", skip_serializing_if = "Option::is_none")]
    pub reported_spam: Option<i32>,
    /// Number of recipients who opted out of your emails using the unsubscribe link in this email.
    #[serde(rename = "unsubscribed", skip_serializing_if = "Option::is_none")]
    pub unsubscribed: Option<i32>,
}

impl PlatformDeliveryDataEmailAllOf {
    pub fn new() -> PlatformDeliveryDataEmailAllOf {
        PlatformDeliveryDataEmailAllOf {
            opened: None,
            unique_opens: None,
            clicks: None,
            unique_clicks: None,
            bounced: None,
            reported_spam: None,
            unsubscribed: None,
        }
    }
}

