use crate::{
    compatibility::stripe::payment_intents::types::StripePaymentMethodOptions,
    core::errors,
    types::{self, api, storage::enums},
};
use serde::{Deserialize, Serialize};

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, Eq, PartialEq)]
pub struct MultisafepayPaymentsRequest {
    #[serde(rename = "type")]
    stype: String,
    gateway: MultisafeGateway,
    order_id: String,
    currency: String,
    amount: String,
    description: Option<String>,
    gateway_info: MultisafepayGatewayInfo
}

#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum MultisafeGateway {
    #[default]
    VISA,
    AMEX,
    CREDITCARD,
    MAESTRO,
    MASTERCARD,
}

#[derive(Default, Clone, Deserialize, Debug, Serialize, Eq, PartialEq)]
pub struct MultisafepayGatewayInfo {
    card_number: String,
    card_expiry_date: String,
    card_holder_name: String,
    card_cvc: String
}

use masking::PeekInterface;

impl TryFrom<&types::PaymentsAuthorizeRouterData> for MultisafepayPaymentsRequest {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(item: &types::PaymentsAuthorizeRouterData) -> Result<Self, Self::Error> {
        match item.request.payment_method_data {
            api::PaymentMethod::Card(ref ccard) => {
                let payment_request = Self {
                    gateway: MultisafeGateway :: CREDITCARD,
                    stype: "direct".to_string(),
                    order_id: item.payment_id.to_string(),
                    currency: item.request.currency.to_string(),
                    amount: item.request.amount.to_string(),
                    description: item.description.clone(),
                    gateway_info: MultisafepayGatewayInfo {
                        card_number: ccard.card_number.peek().clone(),
                        card_expiry_date: format!("{}/{}", ccard.card_exp_month.peek().clone(), ccard.card_exp_year.peek().clone()),
                        card_holder_name: ccard.card_holder_name.peek().clone(),
                        card_cvc: ccard.card_cvc.peek().clone(),
                    }
                };
                Ok(payment_request)
            }
            _ => Err(
                errors::ConnectorError::NotImplemented("Current Payment Method".to_string()).into(),
            ),
        }
    }
}

//TODO: Fill the struct with respective fields
// Auth Struct
pub struct MultisafepayAuthType {
    pub(super) api_key: String,
}

impl TryFrom<&types::ConnectorAuthType> for MultisafepayAuthType {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(_auth_type: &types::ConnectorAuthType) -> Result<Self, Self::Error> {
        todo!()
    }
}
// PaymentsResponse
//TODO: Append the remaining status flags
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MultisafepayPaymentStatus {
    Initialized,
    Uncleared,
    Completed,
    Void,
    Expired,
    #[default]
    Declined,
}

impl From<MultisafepayPaymentStatus> for enums::AttemptStatus {
    fn from(item: MultisafepayPaymentStatus) -> Self {
        match item {
            MultisafepayPaymentStatus::Initialized => Self::Started,
            MultisafepayPaymentStatus::Uncleared => Self::AuthorizationFailed,
            MultisafepayPaymentStatus::Completed => Self::Charged,
            MultisafepayPaymentStatus::Void => Self::Voided,
            MultisafepayPaymentStatus::Expired => Self::Failure,
            MultisafepayPaymentStatus::Declined => Self::AuthenticationFailed,
        }
    }
}
//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultisafepayPaymentsResponse {
    success: bool,
    data: MultisafepayPaymentsData
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultisafepayPaymentsData {
    transaction_id: String,
    order_id: String,
    status: MultisafepayPaymentStatus
}

impl<F, T>
    TryFrom<
        types::ResponseRouterData<F, MultisafepayPaymentsResponse, T, types::PaymentsResponseData>,
    > for types::RouterData<F, T, types::PaymentsResponseData>
{
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(
        item: types::ResponseRouterData<
            F,
            MultisafepayPaymentsResponse,
            T,
            types::PaymentsResponseData,
        >,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            status: enums::AttemptStatus::from(item.response.data.status),
            response: Ok(types::PaymentsResponseData::TransactionResponse {
                resource_id: types::ResponseId::ConnectorTransactionId(item.response.data.order_id),
                redirection_data: None,
                redirect: false,
                mandate_reference: None,
                connector_metadata: None,
            }),
            ..item.data
        })
    }
}

//TODO: Fill the struct with respective fields
// REFUND :
// Type definition for RefundRequest
#[derive(Default, Debug, Serialize)]
pub struct MultisafepayRefundRequest {}

impl<F> TryFrom<&types::RefundsRouterData<F>> for MultisafepayRefundRequest {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(_item: &types::RefundsRouterData<F>) -> Result<Self, Self::Error> {
        todo!()
    }
}

// Type definition for Refund Response

#[allow(dead_code)]
#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub enum RefundStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<RefundStatus> for enums::RefundStatus {
    fn from(item: RefundStatus) -> Self {
        match item {
            RefundStatus::Succeeded => Self::Success,
            RefundStatus::Failed => Self::Failure,
            RefundStatus::Processing => Self::Pending,
            //TODO: Review mapping
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {}

impl TryFrom<types::RefundsResponseRouterData<api::Execute, RefundResponse>>
    for types::RefundsRouterData<api::Execute>
{
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(
        _item: types::RefundsResponseRouterData<api::Execute, RefundResponse>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<types::RefundsResponseRouterData<api::RSync, RefundResponse>>
    for types::RefundsRouterData<api::RSync>
{
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(
        _item: types::RefundsResponseRouterData<api::RSync, RefundResponse>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct MultisafepayErrorResponse {}
