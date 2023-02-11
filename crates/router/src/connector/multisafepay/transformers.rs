use crate::{
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

#[derive(Default, Debug, Serialize, Eq, PartialEq)]
pub struct MultisafepayCaptureRequest {
    amount: Option<i64>,
    new_order_status: String,
}

impl TryFrom<&types::PaymentsCaptureRouterData> for MultisafepayCaptureRequest {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(item: &types::PaymentsCaptureRouterData) -> Result<Self, Self::Error> {
        Ok(Self {
            amount: item.request.amount_to_capture,
            new_order_status: "completed".to_string()
        })
    }
}

//TODO: Fill the struct with respective fields
// Auth Struct
pub struct MultisafepayAuthType {
    pub(super) api_key: String,
}

impl TryFrom<&types::ConnectorAuthType> for MultisafepayAuthType {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(auth_type: &types::ConnectorAuthType) -> Result<Self, Self::Error> {
        if let types::ConnectorAuthType::HeaderKey { api_key } = auth_type {
            Ok(Self {
                api_key: api_key.to_string(),
            })
        } else {
            Err(errors::ConnectorError::FailedToObtainAuthType)?
        }
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

//VOID
#[derive(Default, Debug, Serialize)]
pub struct MultisafepayCancelRequest {
    status: String,
}

impl TryFrom<&types::PaymentsCancelRouterData> for MultisafepayCancelRequest {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(_item: &types::PaymentsCancelRouterData) -> Result<Self, Self::Error> {
        Ok(Self {
            status: "cancelled".to_string(),
        })
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultisafepayCancelResponse {
    success: bool,
}

impl<F, T>
    TryFrom<
        types::ResponseRouterData<F, MultisafepayCancelResponse, T, types::PaymentsResponseData>,
    > for types::RouterData<F, T, types::PaymentsResponseData>
{
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(
        item: types::ResponseRouterData<
            F,
            MultisafepayCancelResponse,
            T,
            types::PaymentsResponseData,
        >,
    ) -> Result<Self, Self::Error> {
        let void_status = 
        match item.response.success {
            true => enums::AttemptStatus::Voided,
            false => enums::AttemptStatus::VoidFailed
        };

        Ok(Self {
            status: void_status,
            response: Ok(types::PaymentsResponseData::TransactionResponse {
                resource_id: types::ResponseId::NoResponseId,
                redirection_data: None,
                redirect: false,
                mandate_reference: None,
                connector_metadata: None,
            }),
            ..item.data
        })
    }
}

// REFUND :
// Type definition for RefundRequest
#[derive(Default, Debug, Serialize)]
pub struct MultisafepayRefundRequest {
    currency: String,
    amount: i64,
}

impl<F> TryFrom<&types::RefundsRouterData<F>> for MultisafepayRefundRequest {
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(item: &types::RefundsRouterData<F>) -> Result<Self, Self::Error> {
        Ok(Self {
            currency: item.request.currency.to_string(),
            amount: item.request.refund_amount,
        })
    }
}

// Type definition for Refund Response
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
    success: bool,
    data: RefundResponseData,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefundResponseData {
    transaction_id: u32,
    refund_id: u32,
}

impl TryFrom<types::RefundsResponseRouterData<api::Execute, RefundResponse>>
    for types::RefundsRouterData<api::Execute>
{
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(
        item: types::RefundsResponseRouterData<api::Execute, RefundResponse>,
    ) -> Result<Self, Self::Error> {
        let refund_status = 
            match item.response.success {
                true => enums::RefundStatus::Pending,
                false => enums::RefundStatus::Failure
            };

        Ok(Self {
            response: Ok(types::RefundsResponseData {
                connector_refund_id: item.response.data.refund_id.to_string().clone(),
                refund_status,
            }),
            ..item.data
        })
    }
}


//TODO: Fill the struct with respective fields
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RefundSyncStatus {
    Reserved,
    Declined,
    #[default]
    Completed,
}

impl From<RefundSyncStatus> for enums::RefundStatus {
    fn from(item: RefundSyncStatus) -> Self {
        match item {
            RefundSyncStatus::Reserved => Self::Pending,
            RefundSyncStatus::Completed => Self::Success,
            RefundSyncStatus::Declined => Self::Failure,
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefundSyncResponse {
    success: bool,
    data: RefundSyncResponseData
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefundSyncResponseData {
    transaction_id: u32,
    order_id: String,
    status: RefundSyncStatus
}

impl TryFrom<types::RefundsResponseRouterData<api::RSync, RefundSyncResponse>>
    for types::RefundsRouterData<api::RSync>
{
    type Error = error_stack::Report<errors::ParsingError>;
    fn try_from(
        item: types::RefundsResponseRouterData<api::RSync, RefundSyncResponse>,
    ) -> Result<Self, Self::Error> {
        let refund_status = enums::RefundStatus::from(item.response.data.status);
        Ok(Self {
            response: Ok(types::RefundsResponseData {
                connector_refund_id: item.response.data.order_id.clone(),
                refund_status,
            }),
            ..item.data
        })
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct MultisafepayErrorResponse {}
