// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_endpoints_by_platform_application_output_next_token(
    input: &crate::output::ListEndpointsByPlatformApplicationOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_origination_numbers_output_next_token(
    input: &crate::output::ListOriginationNumbersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_phone_numbers_opted_out_output_next_token(
    input: &crate::output::ListPhoneNumbersOptedOutOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_platform_applications_output_next_token(
    input: &crate::output::ListPlatformApplicationsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_sms_sandbox_phone_numbers_output_next_token(
    input: &crate::output::ListSmsSandboxPhoneNumbersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_subscriptions_output_next_token(
    input: &crate::output::ListSubscriptionsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_subscriptions_by_topic_output_next_token(
    input: &crate::output::ListSubscriptionsByTopicOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_topics_output_next_token(
    input: &crate::output::ListTopicsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_endpoints_by_platform_application_output_endpoints(
    input: crate::output::ListEndpointsByPlatformApplicationOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Endpoint>> {
    let input = match input.endpoints {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_origination_numbers_output_phone_numbers(
    input: crate::output::ListOriginationNumbersOutput,
) -> std::option::Option<std::vec::Vec<crate::model::PhoneNumberInformation>> {
    let input = match input.phone_numbers {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_phone_numbers_opted_out_output_phone_numbers(
    input: crate::output::ListPhoneNumbersOptedOutOutput,
) -> std::option::Option<std::vec::Vec<std::string::String>> {
    let input = match input.phone_numbers {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_platform_applications_output_platform_applications(
    input: crate::output::ListPlatformApplicationsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::PlatformApplication>> {
    let input = match input.platform_applications {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_sms_sandbox_phone_numbers_output_phone_numbers(
    input: crate::output::ListSmsSandboxPhoneNumbersOutput,
) -> std::option::Option<std::vec::Vec<crate::model::SmsSandboxPhoneNumber>> {
    let input = match input.phone_numbers {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_subscriptions_output_subscriptions(
    input: crate::output::ListSubscriptionsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Subscription>> {
    let input = match input.subscriptions {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_subscriptions_by_topic_output_subscriptions(
    input: crate::output::ListSubscriptionsByTopicOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Subscription>> {
    let input = match input.subscriptions {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_topics_output_topics(
    input: crate::output::ListTopicsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Topic>> {
    let input = match input.topics {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
