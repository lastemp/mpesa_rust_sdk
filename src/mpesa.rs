use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};

use actix_web::web;
use chrono::prelude::*;
use reqwest::header::HeaderMap;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::Response;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::models::{
    AuthTokenResponseData, B2CResultParametersOutputDetails, BusinessBuyGoodsData,
    BusinessBuyGoodsErrorResponseData, BusinessBuyGoodsFailedResultParameter,
    BusinessBuyGoodsInputDetails, BusinessBuyGoodsReferenceItem,
    BusinessBuyGoodsReferenceItemOutputDetails, BusinessBuyGoodsResponseData,
    BusinessBuyGoodsResultParametersOutputDetails, BusinessBuyGoodsTimeoutParametersOutputDetails,
    BusinessPayBillData, BusinessPayBillErrorResponseData, BusinessPayBillFailedResultParameter,
    BusinessPayBillInputDetails, BusinessPayBillReferenceItem,
    BusinessPayBillReferenceItemOutputDetails, BusinessPayBillResponseData,
    BusinessPayBillResultParametersOutputDetails, BusinessPayBillTimeoutParametersOutputDetails,
    BusinessToCustomerData, BusinessToCustomerErrorResponseData, BusinessToCustomerInputDetails,
    BusinessToCustomerResponseData, C2BPaymentResultParametersOutputDetails,
    CustomerToBusinessPaymentData, CustomerToBusinessPaymentErrorResponseData,
    CustomerToBusinessPaymentInputDetails, CustomerToBusinessPaymentResponseData, ItemDetails,
    MixedTypeValue, ReferenceItemDetails, RegisterUrlData, RegisterUrlInputDetails,
    RegisterUrlResponseData, ResultParameter, ResultParameterDetails,
};

const AUTHORISATION_BEARER: &str = "Bearer";

#[derive(Debug)]
pub struct MpesaGateway {
    consumer_key: String,
    consumer_secret: String,
    auth_token_url: String,
}

impl MpesaGateway {
    pub fn new(
        consumer_key: String,
        consumer_secret: String,
        auth_token_url: String,
    ) -> Result<Self, String> {
        if consumer_key.is_empty() || consumer_key.replace(" ", "").trim().len() == 0 {
            return Err(String::from("consumer key is empty"));
        }

        if consumer_secret.is_empty() || consumer_secret.replace(" ", "").trim().len() == 0 {
            return Err(String::from("consumer secret is empty"));
        }

        if auth_token_url.is_empty() || auth_token_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("auth_token url is empty"));
        }

        Ok(Self {
            consumer_key,
            consumer_secret,
            auth_token_url,
        })
    }

    fn get_api_key(&self) -> String {
        let consumer_key = &self.consumer_key;
        let consumer_secret = &self.consumer_secret;
        let mut password: String = consumer_key.to_string();
        let k = ":"; // Separator
        password.push_str(k);
        password.push_str(&consumer_secret);
        let encodedpassword = general_purpose::STANDARD.encode(password);

        let mut api_key = String::from("Basic");
        let k = " "; // Separator
        api_key.push_str(k);
        api_key.push_str(&encodedpassword);

        api_key
    }

    pub fn get_b2c_result_parameters_output_details(
        &self,
        result_parameters: &ResultParameter,
    ) -> B2CResultParametersOutputDetails {
        let mut transaction_amount: f32 = 0.0;
        let mut transaction_receipt = String::from("");
        let mut b2c_recipient_is_registered_customer = String::from("");
        let mut b2c_charges_paid_account_available_funds: f32 = 0.0;
        let mut receiver_party_public_name = String::from("");
        let mut transaction_completed_date_time = String::from("");
        let mut b2c_utility_account_available_funds: f32 = 0.0;
        let mut b2c_working_account_available_funds: f32 = 0.0;

        for result_parameter in result_parameters.ResultParameter.iter() {
            let _key = &result_parameter.Key;
            let _value = &result_parameter.Value;

            //TransactionAmount
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("TransactionAmount"))
            {
                transaction_amount = match _value {
                    MixedTypeValue::StringValue(s) => 0.0,
                    MixedTypeValue::IntegerValue(i) => *i as f32,
                    MixedTypeValue::FloatValue(f) => *f,
                    _ => 0.0,
                }
            }

            //TransactionReceipt
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("TransactionReceipt"))
            {
                transaction_receipt = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            //B2CRecipientIsRegisteredCustomer
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("B2CRecipientIsRegisteredCustomer"))
            {
                b2c_recipient_is_registered_customer = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            //B2CChargesPaidAccountAvailableFunds
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("B2CChargesPaidAccountAvailableFunds"))
            {
                b2c_charges_paid_account_available_funds = match _value {
                    MixedTypeValue::StringValue(s) => 0.0,
                    MixedTypeValue::IntegerValue(i) => *i as f32,
                    MixedTypeValue::FloatValue(f) => *f,
                    _ => 0.0,
                }
            }

            //ReceiverPartyPublicName
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("ReceiverPartyPublicName"))
            {
                receiver_party_public_name = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            //TransactionCompletedDateTime
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("TransactionCompletedDateTime"))
            {
                transaction_completed_date_time = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            //B2CUtilityAccountAvailableFunds
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("B2CUtilityAccountAvailableFunds"))
            {
                b2c_utility_account_available_funds = match _value {
                    MixedTypeValue::StringValue(s) => 0.0,
                    MixedTypeValue::IntegerValue(i) => *i as f32,
                    MixedTypeValue::FloatValue(f) => *f,
                    _ => 0.0,
                }
            }

            //B2CWorkingAccountAvailableFunds
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("B2CWorkingAccountAvailableFunds"))
            {
                b2c_working_account_available_funds = match _value {
                    MixedTypeValue::StringValue(s) => 0.0,
                    MixedTypeValue::IntegerValue(i) => *i as f32,
                    MixedTypeValue::FloatValue(f) => *f,
                    _ => 0.0,
                }
            }
        }

        let b2c_result_parameters_output_details = B2CResultParametersOutputDetails {
            TransactionAmount: transaction_amount,
            TransactionReceipt: transaction_receipt,
            B2CRecipientIsRegisteredCustomer: b2c_recipient_is_registered_customer,
            B2CChargesPaidAccountAvailableFunds: b2c_charges_paid_account_available_funds,
            ReceiverPartyPublicName: receiver_party_public_name,
            TransactionCompletedDateTime: transaction_completed_date_time,
            B2CUtilityAccountAvailableFunds: b2c_utility_account_available_funds,
            B2CWorkingAccountAvailableFunds: b2c_working_account_available_funds,
        };

        b2c_result_parameters_output_details
    }

    pub fn get_c2b_payment_result_parameters_output_details(
        &self,
        list_of_items: &Vec<ItemDetails>,
    ) -> Option<C2BPaymentResultParametersOutputDetails> {
        let mut transaction_amount: f32 = 0.0;
        let mut transaction_receipt = String::from("");
        let mut transaction_date = String::from("");
        let mut phone_number = String::from("");

        let c2b_payment_result_parameters_output_details =
            if !list_of_items.is_empty() && list_of_items.len() > 0 {
                for _item in list_of_items.iter() {
                    let _name = &_item.Name;
                    let _value = &_item.Value;

                    // Amount
                    if _name
                        .to_string()
                        .to_lowercase()
                        .eq_ignore_ascii_case(&String::from("Amount"))
                    {
                        transaction_amount = match _value {
                            MixedTypeValue::StringValue(s) => 0.0,
                            MixedTypeValue::IntegerValue(i) => *i as f32,
                            MixedTypeValue::FloatValue(f) => *f,
                            _ => 0.0,
                        }
                    }

                    // TransactionReceipt
                    if _name
                        .to_string()
                        .to_lowercase()
                        .eq_ignore_ascii_case(&String::from("MpesaReceiptNumber"))
                    {
                        transaction_receipt = match _value {
                            MixedTypeValue::StringValue(s) => s.to_string(),
                            _ => String::from(""),
                        }
                    }

                    //transaction_date
                    if _name
                        .to_string()
                        .to_lowercase()
                        .eq_ignore_ascii_case(&String::from("TransactionDate"))
                    {
                        transaction_date = match _value {
                            MixedTypeValue::FloatValue(f) => f.to_string(),
                            _ => String::from(""),
                        }
                    }

                    // phone_number
                    if _name
                        .to_string()
                        .to_lowercase()
                        .eq_ignore_ascii_case(&String::from("PhoneNumber"))
                    {
                        phone_number = match _value {
                            MixedTypeValue::FloatValue(f) => f.to_string(),
                            _ => String::from(""),
                        }
                    }
                }

                let c2b_payment_result_parameters_output_details =
                    C2BPaymentResultParametersOutputDetails {
                        Amount: transaction_amount,
                        MpesaReceiptNumber: transaction_receipt,
                        TransactionDate: transaction_date,
                        PhoneNumber: phone_number,
                    };

                Some(c2b_payment_result_parameters_output_details)
            } else {
                None
            };

        c2b_payment_result_parameters_output_details
    }

    pub fn get_business_paybill_result_parameters_output_details(
        &self,
        result_parameters: &ResultParameter,
    ) -> BusinessPayBillResultParametersOutputDetails {
        let mut debit_account_balance = String::from("");
        let mut transaction_amount = String::from("");
        let mut debit_party_affected_account_balance = String::from("");
        let mut trans_completed_time = String::from("");
        let mut debit_party_charges = String::from("");
        let mut receiver_party_public_name = String::from("");
        let mut _currency = String::from("");
        let mut initiator_account_current_balance = String::from("");
        let mut bill_reference_number = String::from("");
        let mut queue_timeout_url = String::from("");

        for result_parameter in result_parameters.ResultParameter.iter() {
            let _key = &result_parameter.Key;
            let _value = &result_parameter.Value;

            // DebitAccountBalance
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("DebitAccountBalance"))
            {
                debit_account_balance = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // Amount
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("Amount"))
            {
                transaction_amount = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // DebitPartyAffectedAccountBalance
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("DebitPartyAffectedAccountBalance"))
            {
                debit_party_affected_account_balance = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // TransCompletedTime
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("TransCompletedTime"))
            {
                trans_completed_time = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // DebitPartyCharges
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("DebitPartyCharges"))
            {
                debit_party_charges = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // ReceiverPartyPublicName
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("ReceiverPartyPublicName"))
            {
                receiver_party_public_name = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // Currency
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("Currency"))
            {
                _currency = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // InitiatorAccountCurrentBalance
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("InitiatorAccountCurrentBalance"))
            {
                initiator_account_current_balance = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }
        }

        let business_paybill_result_parameters_output_details =
            BusinessPayBillResultParametersOutputDetails {
                DebitAccountBalance: debit_account_balance,
                Amount: transaction_amount,
                DebitPartyAffectedAccountBalance: debit_party_affected_account_balance,
                TransCompletedTime: trans_completed_time,
                DebitPartyCharges: debit_party_charges,
                ReceiverPartyPublicName: receiver_party_public_name,
                Currency: _currency,
                InitiatorAccountCurrentBalance: initiator_account_current_balance,
            };

        business_paybill_result_parameters_output_details
    }

    pub fn get_business_paybill_Reference_item_output_details(
        &self,
        reference_data: &BusinessPayBillReferenceItem,
    ) -> BusinessPayBillReferenceItemOutputDetails {
        let mut debit_account_balance = String::from("");
        let mut transaction_amount = String::from("");
        let mut debit_party_affected_account_balance = String::from("");
        let mut trans_completed_time = String::from("");
        let mut debit_party_charges = String::from("");
        let mut receiver_party_public_name = String::from("");
        let mut _currency = String::from("");
        let mut initiator_account_current_balance = String::from("");
        let mut bill_reference_number = String::from("");
        let mut queue_timeout_url = String::from("");

        for reference_item in reference_data.ReferenceItem.iter() {
            let _key = &reference_item.Key;
            let _value = &reference_item.Value;

            // BillReferenceNumber
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("BillReferenceNumber"))
            {
                bill_reference_number = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // QueueTimeoutURL
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("QueueTimeoutURL"))
            {
                queue_timeout_url = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }
        }

        let business_paybill_result_parameters_output_details =
            BusinessPayBillReferenceItemOutputDetails {
                BillReferenceNumber: bill_reference_number,
                QueueTimeoutURL: queue_timeout_url,
            };

        business_paybill_result_parameters_output_details
    }

    pub fn get_business_buy_goods_result_parameters_output_details(
        &self,
        result_parameters: &ResultParameter,
    ) -> BusinessBuyGoodsResultParametersOutputDetails {
        let mut debit_account_balance = String::from("");
        let mut transaction_amount = String::from("");
        let mut debit_party_affected_account_balance = String::from("");
        let mut trans_completed_time = String::from("");
        let mut debit_party_charges = String::from("");
        let mut receiver_party_public_name = String::from("");
        let mut _currency = String::from("");
        let mut initiator_account_current_balance = String::from("");
        let mut bill_reference_number = String::from("");
        let mut queue_timeout_url = String::from("");

        for result_parameter in result_parameters.ResultParameter.iter() {
            let _key = &result_parameter.Key;
            let _value = &result_parameter.Value;

            // DebitAccountBalance
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("DebitAccountBalance"))
            {
                debit_account_balance = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // Amount
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("Amount"))
            {
                transaction_amount = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // DebitPartyAffectedAccountBalance
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("DebitPartyAffectedAccountBalance"))
            {
                debit_party_affected_account_balance = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // TransCompletedTime
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("TransCompletedTime"))
            {
                trans_completed_time = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // DebitPartyCharges
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("DebitPartyCharges"))
            {
                debit_party_charges = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // ReceiverPartyPublicName
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("ReceiverPartyPublicName"))
            {
                receiver_party_public_name = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // Currency
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("Currency"))
            {
                _currency = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // InitiatorAccountCurrentBalance
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("InitiatorAccountCurrentBalance"))
            {
                initiator_account_current_balance = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }
        }

        let business_buy_goods_result_parameters_output_details =
            BusinessBuyGoodsResultParametersOutputDetails {
                DebitAccountBalance: debit_account_balance,
                Amount: transaction_amount,
                DebitPartyAffectedAccountBalance: debit_party_affected_account_balance,
                TransCompletedTime: trans_completed_time,
                DebitPartyCharges: debit_party_charges,
                ReceiverPartyPublicName: receiver_party_public_name,
                Currency: _currency,
                InitiatorAccountCurrentBalance: initiator_account_current_balance,
            };

        business_buy_goods_result_parameters_output_details
    }

    pub fn get_business_buy_goods_reference_item_output_details(
        &self,
        reference_data: &BusinessBuyGoodsReferenceItem,
    ) -> BusinessBuyGoodsReferenceItemOutputDetails {
        let mut debit_account_balance = String::from("");
        let mut transaction_amount = String::from("");
        let mut debit_party_affected_account_balance = String::from("");
        let mut trans_completed_time = String::from("");
        let mut debit_party_charges = String::from("");
        let mut receiver_party_public_name = String::from("");
        let mut _currency = String::from("");
        let mut initiator_account_current_balance = String::from("");
        let mut bill_reference_number = String::from("");
        let mut queue_timeout_url = String::from("");

        for reference_item in reference_data.ReferenceItem.iter() {
            let _key = &reference_item.Key;
            let _value = &reference_item.Value;

            // BillReferenceNumber
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("BillReferenceNumber"))
            {
                bill_reference_number = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }

            // QueueTimeoutURL
            if _key
                .to_string()
                .to_lowercase()
                .eq_ignore_ascii_case(&String::from("QueueTimeoutURL"))
            {
                queue_timeout_url = match _value {
                    MixedTypeValue::StringValue(s) => s.to_string(),
                    _ => String::from(""),
                }
            }
        }

        let business_paybill_result_parameters_output_details =
            BusinessBuyGoodsReferenceItemOutputDetails {
                BillReferenceNumber: bill_reference_number,
                QueueTimeoutURL: queue_timeout_url,
            };

        business_paybill_result_parameters_output_details
    }

    pub fn get_business_paybill_timeout_parameters_output_details(
        &self,
        result_parameter: &BusinessPayBillFailedResultParameter,
        reference_data: &ReferenceItemDetails,
    ) -> BusinessPayBillTimeoutParametersOutputDetails {
        let mut bo_completed_time = String::from("");
        let mut queue_timeout_url = String::from("");

        let result_parameter_key = &result_parameter.ResultParameter.Key;
        let result_parameter_value = &result_parameter.ResultParameter.Value;

        let reference_data_key = &reference_data.Key;
        let reference_data_value = &reference_data.Value;

        // BOCompletedTime
        if result_parameter_key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("BOCompletedTime"))
        {
            bo_completed_time = match result_parameter_value {
                MixedTypeValue::FloatValue(s) => s.to_string(),
                _ => String::from(""),
            }
        }

        // QueueTimeoutURL
        if reference_data_key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("QueueTimeoutURL"))
        {
            queue_timeout_url = match reference_data_value {
                s => s.to_string(),
                _ => String::from(""),
            }
        }

        let business_paybill_timeout_parameters_output_details =
            BusinessPayBillTimeoutParametersOutputDetails {
                BOCompletedTime: bo_completed_time,
                QueueTimeoutURL: queue_timeout_url,
            };

        business_paybill_timeout_parameters_output_details
    }

    pub fn get_business_buy_goods_timeout_parameters_output_details(
        &self,
        result_parameter: &BusinessBuyGoodsFailedResultParameter,
        reference_data: &ReferenceItemDetails,
    ) -> BusinessBuyGoodsTimeoutParametersOutputDetails {
        let mut bo_completed_time = String::from("");
        let mut queue_timeout_url = String::from("");

        let result_parameter_key = &result_parameter.ResultParameter.Key;
        let result_parameter_value = &result_parameter.ResultParameter.Value;

        let reference_data_key = &reference_data.Key;
        let reference_data_value = &reference_data.Value;

        // BOCompletedTime
        if result_parameter_key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("BOCompletedTime"))
        {
            bo_completed_time = match result_parameter_value {
                MixedTypeValue::FloatValue(s) => s.to_string(),
                _ => String::from(""),
            }
        }

        // QueueTimeoutURL
        if reference_data_key
            .to_string()
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("QueueTimeoutURL"))
        {
            queue_timeout_url = match reference_data_value {
                s => s.to_string(),
                _ => String::from(""),
            }
        }

        let business_buy_goods_timeout_parameters_output_details =
            BusinessBuyGoodsTimeoutParametersOutputDetails {
                BOCompletedTime: bo_completed_time,
                QueueTimeoutURL: queue_timeout_url,
            };

        business_buy_goods_timeout_parameters_output_details
    }

    /*
    fn build_business_to_customer_response_data(
        &self,
        originator_conversation_id: Option<String>,
        conversation_id: Option<String>,
        response_code: Option<String>,
        response_description: Option<String>,
    ) -> BusinessToCustomerResponseData {
        BusinessToCustomerResponseData {
            OriginatorConversationID: originator_conversation_id,
            ConversationID: conversation_id,
            ResponseCode: response_code,
            ResponseDescription: response_description,
        }
    }

    fn build_business_to_customer_error_response_data(
        &self,
        request_id: Option<String>,
        error_code: Option<String>,
        error_message: Option<String>,
    ) -> BusinessToCustomerErrorResponseData {
        BusinessToCustomerErrorResponseData {
            requestId: request_id,
            errorCode: error_code,
            errorMessage: error_message,
        }
    }
    */

    async fn get_auth_token(&self) -> std::result::Result<String, String> {
        let api_key = self.get_api_key();
        //println!("api_key: {:?}", &api_key);
        let api_url = &self.auth_token_url;
        //println!("api_url: {:?}", &api_url);

        /*
        let xy = tokio::spawn(async move {
            // Process each request concurrently.
            let _access_token = generate_auth_token(api_key, api_url.to_string()).await;
            let access_token: String = match _access_token {
                Ok(a) => a,
                Err(e) => String::from(""),
            };
            return access_token;
        });

        let access_token: String = match xy.await {
            Ok(a) => a,
            Err(e) => String::from(""),
        };

        access_token
        */
        let _result = generate_auth_token(api_key, api_url.to_string()).await;

        _result
        /*
        let access_token: String = match _result {
            Ok(a) => {
                if !a.is_empty() {
                    let mut access_token = AUTHORISATION_BEARER.to_string();
                    let k = " "; // Separator
                    access_token.push_str(k);
                    access_token.push_str(&a);

                    access_token
                } else {
                    String::from("")
                }
            }
            Err(e) => String::from(""),
        };

        access_token
        */
    }

    fn parse_auth_token(&self, access_token_result: String) -> String {
        /*
        let access_token: String = match access_token_result {
            Ok(a) => {
                if !a.is_empty() {
                    let mut access_token = AUTHORISATION_BEARER.to_string();
                    let k = " "; // Separator
                    access_token.push_str(k);
                    access_token.push_str(&a);

                    access_token
                } else {
                    String::from("")
                }
            }
            Err(e) => String::from(""),
        };
        */
        let access_token: String = if !access_token_result.is_empty()
            && access_token_result.replace(" ", "").trim().len() > 0
        {
            let mut access_token = AUTHORISATION_BEARER.to_string();
            let k = " "; // Separator
            access_token.push_str(k);
            access_token.push_str(&access_token_result);

            access_token
        } else {
            String::from("")
        };

        access_token
    }

    pub async fn get_register_url(
        &self,
        register_url_details: RegisterUrlInputDetails,
    ) -> std::result::Result<RegisterUrlResponseData, reqwest::Error> {
        /*
        let _output = self.get_auth_token();
        let access_token: String = _output.await;
        //let api_url = &self.register_url;
        //println!("access_token: {:?}", &access_token);
        let register_url_response_data = build_register_url_response_data(None, None, None);

        if access_token.is_empty() || access_token.replace(" ", "").trim().len() == 0 {
            //println!("access_token or api_url or register_url_details is empty");
            println!("access_token is empty");
            /*
            let b = RegisterUrlResponseData {
                OriginatorCoversationID: None,
                ConversationID: None,
                ResponseDescription: None,
            };
            return b;
            */
            return Ok(register_url_response_data);
        }
        */

        /*
        let _result = register_url(register_url_details, access_token).await;

        _result
        */

        let _output = self.get_auth_token();

        let _result = _output.await;

        let output_result = if let Ok(access_token_result) = _result {
            let access_token: String = self.parse_auth_token(access_token_result);

            let _result = register_url(register_url_details, access_token).await;

            _result
        } else if let Err(e) = _result {
            //println!("Data Error: {:?}", e);
            let _x = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let _y = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let mut _z = String::from("generate oauth: ");
            _z.push_str(&e);
            let register_url_response_data =
                build_register_url_response_data(Some(_x), Some(_y), Some(_z));
            Ok(register_url_response_data)
        } else {
            //println!("Unexpected error occured during processing");
            let _x = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let _y = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let mut _z = String::from("generate oauth: ");
            _z.push_str(&String::from("Unexpected error occured during processing"));
            let register_url_response_data =
                build_register_url_response_data(Some(_x), Some(_y), Some(_z));
            Ok(register_url_response_data)
        };

        output_result
    }

    pub async fn get_b2c(
        &self,
        business_to_customer_details: BusinessToCustomerInputDetails,
    ) -> std::result::Result<
        (
            BusinessToCustomerResponseData,
            BusinessToCustomerErrorResponseData,
        ),
        reqwest::Error,
    > {
        /*
        let _output = self.get_auth_token();
        let access_token: String = _output.await;

        let business_to_customer_response_data =
            build_business_to_customer_response_data(None, None, None, None);

        let business_to_customer_error_response_data =
            build_business_to_customer_error_response_data(None, None, None);

        let my_output = (
            business_to_customer_response_data,
            business_to_customer_error_response_data,
        );

        if access_token.is_empty() || access_token.replace(" ", "").trim().len() == 0 {
            println!("access_token is empty");
            return Ok(my_output);
        }

        let _result = business_to_customer(business_to_customer_details, access_token).await;

        _result
        */

        //

        let _output = self.get_auth_token();

        let _result = _output.await;

        let output_result = if let Ok(access_token_result) = _result {
            let access_token: String = self.parse_auth_token(access_token_result);

            let _result = business_to_customer(business_to_customer_details, access_token).await;

            _result
        } else if let Err(e) = _result {
            //println!("Data Error: {:?}", e);
            let _x = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let _y = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let mut _z = String::from("generate oauth: ");
            _z.push_str(&e);

            let business_to_customer_response_data =
                build_business_to_customer_response_data(None, None, None, None);

            let business_to_customer_error_response_data =
                build_business_to_customer_error_response_data(Some(_x), Some(_y), Some(_z));

            let my_output = (
                business_to_customer_response_data,
                business_to_customer_error_response_data,
            );

            Ok(my_output)
        } else {
            //println!("Unexpected error occured during processing");
            let _x = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let _y = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let mut _z = String::from("generate oauth: ");
            _z.push_str(&String::from("Unexpected error occured during processing"));

            let business_to_customer_response_data =
                build_business_to_customer_response_data(None, None, None, None);

            let business_to_customer_error_response_data =
                build_business_to_customer_error_response_data(Some(_x), Some(_y), Some(_z));

            let my_output = (
                business_to_customer_response_data,
                business_to_customer_error_response_data,
            );

            Ok(my_output)
        };

        output_result
    }

    pub async fn get_c2b_payment(
        &self,
        customer_to_business_details: CustomerToBusinessPaymentInputDetails,
    ) -> std::result::Result<
        (
            CustomerToBusinessPaymentResponseData,
            CustomerToBusinessPaymentErrorResponseData,
        ),
        reqwest::Error,
    > {
        /*
        let _output = self.get_auth_token();
        let access_token: String = _output.await;
        //println!("access_token: {:?}", &access_token);
        let customer_to_business_response_data =
            build_customer_to_business_payment_response_data(None, None, None, None, None);

        let customer_to_business_error_response_data =
            build_customer_to_business_payment_error_response_data(None, None, None);

        let my_output = (
            customer_to_business_response_data,
            customer_to_business_error_response_data,
        );

        if access_token.is_empty() || access_token.replace(" ", "").trim().len() == 0 {
            println!("access_token is empty");

            return Ok(my_output);
        }

        let _result =
            customer_to_business_payment(customer_to_business_details, access_token).await;

        _result
        */

        let _output = self.get_auth_token();

        let _result = _output.await;

        let output_result = if let Ok(access_token_result) = _result {
            let access_token: String = self.parse_auth_token(access_token_result);

            let _result =
                customer_to_business_payment(customer_to_business_details, access_token).await;

            _result
        } else if let Err(e) = _result {
            //println!("Data Error: {:?}", e);
            let _x = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let _y = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let mut _z = String::from("generate oauth: ");
            _z.push_str(&e);

            let customer_to_business_response_data =
                build_customer_to_business_payment_response_data(None, None, None, None, None);

            let customer_to_business_error_response_data =
                build_customer_to_business_payment_error_response_data(
                    Some(_x),
                    Some(_y),
                    Some(_z),
                );

            let my_output = (
                customer_to_business_response_data,
                customer_to_business_error_response_data,
            );

            Ok(my_output)
        } else {
            //println!("Unexpected error occured during processing");
            let _x = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let _y = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let mut _z = String::from("generate oauth: ");
            _z.push_str(&String::from("Unexpected error occured during processing"));

            let customer_to_business_response_data =
                build_customer_to_business_payment_response_data(None, None, None, None, None);

            let customer_to_business_error_response_data =
                build_customer_to_business_payment_error_response_data(
                    Some(_x),
                    Some(_y),
                    Some(_z),
                );

            let my_output = (
                customer_to_business_response_data,
                customer_to_business_error_response_data,
            );

            Ok(my_output)
        };

        output_result
    }

    pub async fn get_business_paybill(
        &self,
        business_paybill_details: BusinessPayBillInputDetails,
    ) -> std::result::Result<
        (
            BusinessPayBillResponseData,
            BusinessPayBillErrorResponseData,
        ),
        reqwest::Error,
    > {
        /*
        let _output = self.get_auth_token();
        let access_token: String = _output.await;
        //println!("access_token: {:?}", &access_token);

        let business_paybill_response_data =
            build_business_paybill_response_data(None, None, None, None);

        let business_paybill_error_response_data =
            build_business_paybill_error_response_data(None, None, None);

        let my_output = (
            business_paybill_response_data,
            business_paybill_error_response_data,
        );

        if access_token.is_empty() || access_token.replace(" ", "").trim().len() == 0 {
            println!("access_token is empty");
            return Ok(my_output);
        }

        let _result = business_paybill(business_paybill_details, access_token).await;

        _result
        */
        let _output = self.get_auth_token();

        let _result = _output.await;

        let output_result = if let Ok(access_token_result) = _result {
            let access_token: String = self.parse_auth_token(access_token_result);

            let _result = business_paybill(business_paybill_details, access_token).await;

            _result
        } else if let Err(e) = _result {
            //println!("Data Error: {:?}", e);
            let _x = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let _y = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let mut _z = String::from("generate oauth: ");
            _z.push_str(&e);

            let business_paybill_response_data =
                build_business_paybill_response_data(None, None, None, None);

            let business_paybill_error_response_data =
                build_business_paybill_error_response_data(Some(_x), Some(_y), Some(_z));

            let my_output = (
                business_paybill_response_data,
                business_paybill_error_response_data,
            );

            Ok(my_output)
        } else {
            //println!("Unexpected error occured during processing");
            let _x = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let _y = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let mut _z = String::from("generate oauth: ");
            _z.push_str(&String::from("Unexpected error occured during processing"));

            let business_paybill_response_data =
                build_business_paybill_response_data(None, None, None, None);

            let business_paybill_error_response_data =
                build_business_paybill_error_response_data(Some(_x), Some(_y), Some(_z));

            let my_output = (
                business_paybill_response_data,
                business_paybill_error_response_data,
            );

            Ok(my_output)
        };

        output_result
    }

    pub async fn get_business_buy_goods(
        &self,
        business_buy_goods_details: BusinessBuyGoodsInputDetails,
    ) -> std::result::Result<
        (
            BusinessBuyGoodsResponseData,
            BusinessBuyGoodsErrorResponseData,
        ),
        reqwest::Error,
    > {
        /*
        let _output = self.get_auth_token();
        let access_token: String = _output.await;
        //println!("access_token: {:?}", &access_token);
        let business_buy_goods_response_data =
            build_business_buy_goods_response_data(None, None, None, None);

        let business_buy_goods_error_response_data =
            build_business_buy_goods_error_response_data(None, None, None);

        let my_output = (
            business_buy_goods_response_data,
            business_buy_goods_error_response_data,
        );

        if access_token.is_empty() || access_token.replace(" ", "").trim().len() == 0 {
            println!("access_token is empty");

            return Ok(my_output);
        }

        let _result = business_buy_goods(business_buy_goods_details, access_token).await;

        _result
        */

        let _output = self.get_auth_token();

        let _result = _output.await;

        let output_result = if let Ok(access_token_result) = _result {
            let access_token: String = self.parse_auth_token(access_token_result);

            let _result = business_buy_goods(business_buy_goods_details, access_token).await;

            _result
        } else if let Err(e) = _result {
            //println!("Data Error: {:?}", e);
            let _x = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let _y = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let mut _z = String::from("generate oauth: ");
            _z.push_str(&e);

            let business_buy_goods_response_data =
                build_business_buy_goods_response_data(None, None, None, None);

            let business_buy_goods_error_response_data =
                build_business_buy_goods_error_response_data(Some(_x), Some(_y), Some(_z));

            let my_output = (
                business_buy_goods_response_data,
                business_buy_goods_error_response_data,
            );

            Ok(my_output)
        } else {
            //println!("Unexpected error occured during processing");
            let _x = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let _y = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
            let mut _z = String::from("generate oauth: ");
            _z.push_str(&String::from("Unexpected error occured during processing"));

            let business_buy_goods_response_data =
                build_business_buy_goods_response_data(None, None, None, None);

            let business_buy_goods_error_response_data =
                build_business_buy_goods_error_response_data(Some(_x), Some(_y), Some(_z));

            let my_output = (
                business_buy_goods_response_data,
                business_buy_goods_error_response_data,
            );

            Ok(my_output)
        };

        output_result
    }
}

fn build_business_to_customer_response_data(
    originator_conversation_id: Option<String>,
    conversation_id: Option<String>,
    response_code: Option<String>,
    response_description: Option<String>,
) -> BusinessToCustomerResponseData {
    BusinessToCustomerResponseData {
        OriginatorConversationID: originator_conversation_id,
        ConversationID: conversation_id,
        ResponseCode: response_code,
        ResponseDescription: response_description,
    }
}

fn build_business_to_customer_error_response_data(
    request_id: Option<String>,
    error_code: Option<String>,
    error_message: Option<String>,
) -> BusinessToCustomerErrorResponseData {
    BusinessToCustomerErrorResponseData {
        requestId: request_id,
        errorCode: error_code,
        errorMessage: error_message,
    }
}

fn build_customer_to_business_payment_response_data(
    merchant_request_id: Option<String>,
    checkout_request_id: Option<String>,
    response_code: Option<String>,
    response_description: Option<String>,
    customer_message: Option<String>,
) -> CustomerToBusinessPaymentResponseData {
    CustomerToBusinessPaymentResponseData {
        MerchantRequestID: merchant_request_id,
        CheckoutRequestID: checkout_request_id,
        ResponseCode: response_code,
        ResponseDescription: response_description,
        CustomerMessage: customer_message,
    }
}

fn build_customer_to_business_payment_error_response_data(
    request_id: Option<String>,
    error_code: Option<String>,
    error_message: Option<String>,
) -> CustomerToBusinessPaymentErrorResponseData {
    CustomerToBusinessPaymentErrorResponseData {
        requestId: request_id,
        errorCode: error_code,
        errorMessage: error_message,
    }
}

fn build_business_paybill_response_data(
    originator_conversation_id: Option<String>,
    conversation_id: Option<String>,
    response_code: Option<String>,
    response_description: Option<String>,
) -> BusinessPayBillResponseData {
    BusinessPayBillResponseData {
        OriginatorConversationID: originator_conversation_id,
        ConversationID: conversation_id,
        ResponseCode: response_code,
        ResponseDescription: response_description,
    }
}

fn build_business_paybill_error_response_data(
    request_id: Option<String>,
    error_code: Option<String>,
    error_message: Option<String>,
) -> BusinessPayBillErrorResponseData {
    BusinessPayBillErrorResponseData {
        requestId: request_id,
        errorCode: error_code,
        errorMessage: error_message,
    }
}

fn build_business_buy_goods_response_data(
    originator_conversation_id: Option<String>,
    conversation_id: Option<String>,
    response_code: Option<String>,
    response_description: Option<String>,
) -> BusinessBuyGoodsResponseData {
    BusinessBuyGoodsResponseData {
        OriginatorConversationID: originator_conversation_id,
        ConversationID: conversation_id,
        ResponseCode: response_code,
        ResponseDescription: response_description,
    }
}

fn build_business_buy_goods_error_response_data(
    request_id: Option<String>,
    error_code: Option<String>,
    error_message: Option<String>,
) -> BusinessBuyGoodsErrorResponseData {
    BusinessBuyGoodsErrorResponseData {
        requestId: request_id,
        errorCode: error_code,
        errorMessage: error_message,
    }
}

fn build_register_url_data(
    short_code: String,
    response_type: String,
    confirmation_url: String,
    validation_url: String,
) -> RegisterUrlData {
    RegisterUrlData {
        ShortCode: short_code,
        ResponseType: response_type,
        ConfirmationURL: confirmation_url,
        ValidationURL: validation_url,
    }
}

fn build_register_url_response_data(
    originator_conversation_id: Option<String>,
    conversation_id: Option<String>,
    response_description: Option<String>,
) -> RegisterUrlResponseData {
    RegisterUrlResponseData {
        OriginatorCoversationID: originator_conversation_id,
        ConversationID: conversation_id,
        ResponseDescription: response_description,
    }
}

fn build_business_to_customer_data(
    originator_conversation_id: String,
    initiator_name: String,
    security_credential: String,
    command_id: String,
    amount: u32,
    party_a: u32,
    party_b: String,
    _remarks: String,
    queue_time_out_url: String,
    result_url: String,
    _occassion: String,
) -> BusinessToCustomerData {
    BusinessToCustomerData {
        OriginatorConversationID: originator_conversation_id,
        InitiatorName: initiator_name,
        SecurityCredential: security_credential,
        CommandID: command_id,
        Amount: amount,
        PartyA: party_a,
        PartyB: party_b,
        Remarks: _remarks,
        QueueTimeOutURL: queue_time_out_url,
        ResultURL: result_url,
        Occassion: _occassion,
    }
}

fn build_customer_to_business_data(
    business_short_code: String,
    _password: String,
    time_stamp: String,
    transaction_type: String,
    _amount: u32,
    party_a: u64,
    party_b: u32,
    phone_number: u64,
    call_back_url: String,
    account_reference: String,
    transaction_desc: String,
) -> CustomerToBusinessPaymentData {
    CustomerToBusinessPaymentData {
        BusinessShortCode: business_short_code,
        Password: _password,
        Timestamp: time_stamp,
        TransactionType: transaction_type,
        Amount: _amount,
        PartyA: party_a,
        PartyB: party_b,
        PhoneNumber: phone_number,
        CallBackURL: call_back_url,
        AccountReference: account_reference,
        TransactionDesc: transaction_desc,
    }
}

fn build_business_paybill_data(
    _initiator: String,
    security_credential: String,
    command_id: String,
    sender_identifier_type: String,
    reciever_identifier_type: String,
    _amount: u32,
    party_a: String,
    party_b: String,
    account_reference: String,
    _requester: String,
    _remarks: String,
    queue_time_out_url: String,
    result_url: String,
) -> BusinessPayBillData {
    BusinessPayBillData {
        Initiator: _initiator,
        SecurityCredential: security_credential,
        CommandID: command_id,
        SenderIdentifierType: sender_identifier_type,
        RecieverIdentifierType: reciever_identifier_type,
        Amount: _amount,
        PartyA: party_a,
        PartyB: party_b,
        AccountReference: account_reference,
        Requester: _requester,
        Remarks: _remarks,
        QueueTimeOutURL: queue_time_out_url,
        ResultURL: result_url,
    }
}

fn build_business_buy_goods_data(
    _initiator: String,
    security_credential: String,
    command_id: String,
    sender_identifier_type: String,
    reciever_identifier_type: String,
    _amount: u32,
    party_a: String,
    party_b: String,
    account_reference: String,
    _requester: String,
    _remarks: String,
    queue_time_out_url: String,
    result_url: String,
) -> BusinessBuyGoodsData {
    BusinessBuyGoodsData {
        Initiator: _initiator,
        SecurityCredential: security_credential,
        CommandID: command_id,
        SenderIdentifierType: sender_identifier_type,
        RecieverIdentifierType: reciever_identifier_type,
        Amount: _amount,
        PartyA: party_a,
        PartyB: party_b,
        AccountReference: account_reference,
        Requester: _requester,
        Remarks: _remarks,
        QueueTimeOutURL: queue_time_out_url,
        ResultURL: result_url,
    }
}

fn build_headers_generate_auth_token(api_key: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, "text/plain".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", api_key.parse().unwrap());

    headers
}

fn build_headers(access_token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", access_token.parse().unwrap());

    headers
}

async fn parse_auth_token_response_data(
    response: Response,
) -> std::result::Result<AuthTokenResponseData, reqwest::Error> {
    let auth_token_response_data = response.json::<AuthTokenResponseData>().await?;

    Ok(auth_token_response_data)
}

async fn generate_auth_token(
    api_key: String,
    api_url: String,
) -> std::result::Result<String, String> {
    let client = reqwest::Client::new();
    let mut access_token = String::from("");
    // "%Y-%m-%d %H:%M:%S" i.e "yyyy-MM-dd HH:mm:ss"
    // "%Y-%m-%d %H:%M:%S%.3f" i.e "yyyy-MM-dd HH:mm:ss.SSS"
    let date_to_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let res = client
        .get(api_url)
        /*
        .header(CONTENT_TYPE, "text/plain")
        .header(ACCEPT, "application/json")
        .header("Authorization", api_key)
        */
        .headers(build_headers_generate_auth_token(api_key))
        .send()
        //.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
        .await;

    match res {
        Err(e) => {
            //println!("server not responding");
            return Err(e.to_string());
        }
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    //let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.

                    //let my_output = response.json::<AuthTokenResponseData>().await?;

                    //let access_token = &my_output.access_token.as_ref().unwrap_or(&k);
                    /*
                    let _access_token = &my_output.access_token.as_ref().unwrap_or(&k);
                    access_token = _access_token.to_string();
                    */

                    /*
                    let _result: std::result::Result<AuthTokenResponseData, reqwest::Error> =
                        Ok(response.json::<AuthTokenResponseData>().await?);
                    */

                    let _output = parse_auth_token_response_data(response);
                    let _result = _output.await;
                    /*
                    let access_token: String = match _result {
                        Ok(_x) => {
                            let _access_token = _x.access_token.as_ref().unwrap_or(&k);
                            _access_token.to_string()
                        }
                        Err(e) => e.to_string(),
                    };
                    */
                    if let Ok(_x) = _result {
                        let _access_token = _x.access_token.as_ref().unwrap_or(&k);
                        let access_token = _access_token.to_string();

                        return Ok(access_token);
                    } else if let Err(e) = _result {
                        let err_data = e.to_string();
                        return Err(err_data);
                    } else {
                        let err_data = String::from("auth token failed to be generated");
                        return Err(err_data);
                    };

                    //return Ok(access_token);
                }
                s => {
                    //println!("Received response status: {:?}", s);
                    return Err(response.status().to_string());
                }
            }
        }
    };

    Ok(access_token)
}

pub async fn register_url(
    register_url_details: RegisterUrlInputDetails,
    access_token: String,
) -> std::result::Result<RegisterUrlResponseData, reqwest::Error> {
    /*
    let short_code: String = register_url_details.short_code;
    let response_type: String = register_url_details.response_type;
    let confirmation_url: String = register_url_details.confirmation_url;
    let validation_url: String = register_url_details.validation_url;
    */
    /*
    let register_url_data = RegisterUrlData {
        ShortCode: short_code,
        ResponseType: response_type,
        ConfirmationURL: confirmation_url,
        ValidationURL: validation_url,
    };
    */
    let api_url: String = register_url_details.get_api_url();
    let short_code: String = register_url_details.get_short_code();
    let response_type: String = register_url_details.get_response_type();
    let confirmation_url: String = register_url_details.get_confirmation_url();
    let validation_url: String = register_url_details.get_validation_url();

    let register_url_data =
        build_register_url_data(short_code, response_type, confirmation_url, validation_url);

    let client = reqwest::Client::new();

    let register_url_response_data = build_register_url_response_data(None, None, None);
    //println!("register_url_data: {:?}", &register_url_data);
    //println!("access_token: {:?}", &access_token);
    // "%Y-%m-%d %H:%M:%S" i.e "yyyy-MM-dd HH:mm:ss"
    // "%Y-%m-%d %H:%M:%S%.3f" i.e "yyyy-MM-dd HH:mm:ss.SSS"
    //let date_to_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let res = client
        .post(api_url)
        /*
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", access_token)
        */
        .headers(build_headers(access_token))
        .json(&register_url_data)
        .send()
        //.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
        .await;

    match res {
        Err(e) => {
            println!("server not responding");
            return Err(e);
        }
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    let register_url_response_data =
                        response.json::<RegisterUrlResponseData>().await?;

                    return Ok(register_url_response_data);
                    //let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    /*
                    let k = String::from(""); //Default value.

                    let my_output = response.json::<RegisterUrlResponseData>().await?;

                    let originator_coversation_id =
                        &my_output.OriginatorCoversationID.as_ref().unwrap_or(&k);
                    let conversation_id = &my_output.ConversationID.as_ref().unwrap_or(&k);
                    let response_description =
                        &my_output.ResponseDescription.as_ref().unwrap_or(&k);
                    register_url_response_data.OriginatorCoversationID =
                        Some(originator_coversation_id.to_string());
                    register_url_response_data.ConversationID = Some(conversation_id.to_string());
                    register_url_response_data.ResponseDescription =
                        Some(response_description.to_string());
                    */
                    /*
                    create_mpesa_register_url(
                        &data,
                        originator_coversation_id.to_string(),
                        conversation_id.to_string(),
                        response_description.to_string(),
                        register_url_data.ShortCode,
                        register_url_data.ConfirmationURL,
                        register_url_data.ValidationURL,
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
                s => {
                    println!("Received response status: {:?}", s);
                    let _x = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
                    let _y = Local::now().format("%Y%m%d%H%M%S%3f").to_string();
                    let status_code: String = response.status().to_string();

                    let originator_conversation_id: Option<String> = Some(_x);
                    let conversation_id: Option<String> = Some(_y);
                    let response_description: Option<String> = Some(status_code);

                    let register_url_response_data = build_register_url_response_data(
                        originator_conversation_id,
                        conversation_id,
                        response_description,
                    );

                    return Ok(register_url_response_data);
                }
            }
        }
    };

    Ok(register_url_response_data)
}

pub async fn business_to_customer(
    business_to_customer_details: BusinessToCustomerInputDetails,
    access_token: String,
) -> std::result::Result<
    (
        BusinessToCustomerResponseData,
        BusinessToCustomerErrorResponseData,
    ),
    reqwest::Error,
> {
    let api_url: String = business_to_customer_details.get_api_url();
    let originator_conversation_id = business_to_customer_details.get_originator_conversation_id();
    let initiator_name: String = business_to_customer_details.get_initiator_name();
    let security_credential: String = business_to_customer_details.get_security_credential();
    let command_id: String = business_to_customer_details.get_command_id();
    let amount: u32 = business_to_customer_details.get_amount();
    let party_a: u32 = business_to_customer_details.get_party_a();
    let party_b: String = business_to_customer_details.get_party_b();
    let _remarks: String = business_to_customer_details.get_remarks();
    let queue_time_out_url: String = business_to_customer_details.get_queue_time_out_url();
    let result_url: String = business_to_customer_details.get_result_url();
    let _occassion: String = business_to_customer_details.get_occassion();
    /*
    let business_to_customer_data = BusinessToCustomerData {
        InitiatorName: initiator_name,
        SecurityCredential: security_credential,
        CommandID: command_id,
        Amount: amount,
        PartyA: party_a,
        PartyB: party_b,
        Remarks: _remarks,
        QueueTimeOutURL: queue_time_out_url,
        ResultURL: result_url,
        Occassion: _occassion,
    };
    */
    let business_to_customer_data = build_business_to_customer_data(
        originator_conversation_id,
        initiator_name,
        security_credential,
        command_id,
        amount,
        party_a,
        party_b,
        _remarks,
        queue_time_out_url,
        result_url,
        _occassion,
    );
    let business_to_customer_response_data =
        build_business_to_customer_response_data(None, None, None, None);

    let business_to_customer_error_response_data =
        build_business_to_customer_error_response_data(None, None, None);

    /*
    println!("access_token: {:?}", &access_token);
    println!(
        "business_to_customer_data: {:?}",
        &business_to_customer_data
    );
    */
    let client = reqwest::Client::new();
    // "%Y-%m-%d %H:%M:%S" i.e "yyyy-MM-dd HH:mm:ss"
    // "%Y-%m-%d %H:%M:%S%.3f" i.e "yyyy-MM-dd HH:mm:ss.SSS"
    //let date_to_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let res = client
        .post(api_url)
        /*
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", access_token)
        */
        .headers(build_headers(access_token))
        .json(&business_to_customer_data)
        .send()
        //.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
        .await;

    match res {
        Err(e) => {
            //println!("server not responding");
            println!("server not responding: {:?}", e.to_string());
            return Err(e);
        }
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    //let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    //let k = String::from(""); //Default value.

                    let business_to_customer_response_data =
                        response.json::<BusinessToCustomerResponseData>().await?;
                    /*
                    let originator_conversation_id = &business_to_customer_response_data
                        .OriginatorConversationID
                        .as_ref()
                        .unwrap_or(&k);
                    let conversation_id = &business_to_customer_response_data
                        .ConversationID
                        .as_ref()
                        .unwrap_or(&k);
                    let response_code = &business_to_customer_response_data
                        .ResponseCode
                        .as_ref()
                        .unwrap_or(&k);
                    let response_description = &business_to_customer_response_data
                        .ResponseDescription
                        .as_ref()
                        .unwrap_or(&k);
                    let request_id = String::from("");
                    let error_code = String::from("");
                    let error_message = String::from("");
                    */

                    let my_output = (
                        business_to_customer_response_data,
                        business_to_customer_error_response_data,
                    );

                    return Ok(my_output);

                    /*
                    business_to_customer_response_data.OriginatorConversationID =
                        Some(originator_conversation_id.to_string());
                    business_to_customer_response_data.ConversationID =
                        Some(conversation_id.to_string());
                    business_to_customer_response_data.ResponseCode =
                        Some(response_code.to_string());
                    business_to_customer_response_data.ResponseDescription =
                        Some(response_description.to_string());

                    */

                    /*
                    println!(
                        "business_to_customer_response_data: {:?}",
                        &business_to_customer_response_data
                    );
                    */

                    /*
                    create_b2c_acknowledgement(
                        &data,
                        originator_conversation_id.to_string(),
                        conversation_id.to_string(),
                        response_code.to_string(),
                        response_description.to_string(),
                        business_to_customer_data.CommandID,
                        business_to_customer_data.PartyA,
                        business_to_customer_data.PartyB,
                        business_to_customer_data.Amount,
                        request_id,
                        error_code,
                        error_message,
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
                s => {
                    //let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    //let k = String::from(""); //Default value.
                    let business_to_customer_error_response_data = response
                        .json::<BusinessToCustomerErrorResponseData>()
                        .await?;
                    /*
                    let request_id = &business_to_customer_error_response_data.requestId.as_ref().unwrap_or(&k);
                    let error_code = &business_to_customer_error_response_data.errorCode.as_ref().unwrap_or(&k);
                    let error_message = &business_to_customer_error_response_data.errorMessage.as_ref().unwrap_or(&k);
                    let originator_conversation_id = String::from("");
                    let conversation_id = String::from("");
                    let response_code = String::from("");
                    let response_description = String::from("");

                    println!("business_to_customer_error_response_data: {:?}", &business_to_customer_error_response_data);

                    business_to_customer_response_data.OriginatorConversationID =
                        Some(originator_conversation_id.to_string());
                    business_to_customer_response_data.ConversationID =
                        Some(request_id.to_string());
                    business_to_customer_response_data.ResponseCode = Some(error_code.to_string());
                    business_to_customer_response_data.ResponseDescription =
                        Some(error_message.to_string());
                    */

                    let my_output = (
                        business_to_customer_response_data,
                        business_to_customer_error_response_data,
                    );

                    return Ok(my_output);

                    /*
                    create_b2c_acknowledgement(
                        &data,
                        originator_conversation_id.to_string(),
                        conversation_id.to_string(),
                        response_code.to_string(),
                        response_description.to_string(),
                        business_to_customer_data.CommandID,
                        business_to_customer_data.PartyA,
                        business_to_customer_data.PartyB,
                        business_to_customer_data.Amount,
                        request_id.to_string(),
                        error_code.to_string(),
                        error_message.to_string(),
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
            }
        }
    };

    let my_output = (
        business_to_customer_response_data,
        business_to_customer_error_response_data,
    );

    Ok(my_output)
}

// network initiated push
pub async fn customer_to_business_payment(
    customer_to_business_payment_details: CustomerToBusinessPaymentInputDetails,
    access_token: String,
) -> std::result::Result<
    (
        CustomerToBusinessPaymentResponseData,
        CustomerToBusinessPaymentErrorResponseData,
    ),
    reqwest::Error,
> {
    let api_url: String = customer_to_business_payment_details.get_api_url();
    let business_short_code: String =
        customer_to_business_payment_details.get_business_short_code();
    let _password: String = customer_to_business_payment_details.get_password();
    let time_stamp: String = customer_to_business_payment_details.get_time_stamp();
    let transaction_type: String = customer_to_business_payment_details.get_transaction_type();
    let _amount: u32 = customer_to_business_payment_details.get_amount();
    let party_a: u64 = customer_to_business_payment_details.get_party_a();
    let party_b: u32 = customer_to_business_payment_details.get_party_b();
    let phone_number: u64 = customer_to_business_payment_details.get_phone_number();
    let call_back_url: String = customer_to_business_payment_details.get_call_back_url();
    let account_reference: String = customer_to_business_payment_details.get_account_reference();
    let transaction_desc: String = customer_to_business_payment_details.get_transaction_desc();
    /*
    let customer_to_business_data = CustomerToBusinessPaymentData {
        BusinessShortCode: business_short_code,
        Password: _password,
        Timestamp: time_stamp,
        TransactionType: transaction_type,
        Amount: _amount,
        PartyA: party_a,
        PartyB: party_b,
        PhoneNumber: phone_number,
        CallBackURL: call_back_url,
        AccountReference: account_reference,
        TransactionDesc: transaction_desc,
    };
    */
    let customer_to_business_data = build_customer_to_business_data(
        business_short_code,
        _password,
        time_stamp,
        transaction_type,
        _amount,
        party_a,
        party_b,
        phone_number,
        call_back_url,
        account_reference,
        transaction_desc,
    );
    let customer_to_business_response_data =
        build_customer_to_business_payment_response_data(None, None, None, None, None);

    let customer_to_business_error_response_data =
        build_customer_to_business_payment_error_response_data(None, None, None);

    /*
    println!("access_token: {:?}", &access_token);
    println!("api_url: {:?}", &api_url);
    println!(
        "customer_to_business_data: {:?}",
        &customer_to_business_data
    );
    */
    let client = reqwest::Client::new();
    // "%Y-%m-%d %H:%M:%S" i.e "yyyy-MM-dd HH:mm:ss"
    // "%Y-%m-%d %H:%M:%S%.3f" i.e "yyyy-MM-dd HH:mm:ss.SSS"
    //let date_to_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let res = client
        .post(api_url)
        /*
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", access_token)
        */
        .headers(build_headers(access_token))
        .json(&customer_to_business_data)
        .send()
        //.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
        .await;

    match res {
        Err(e) => {
            //println!("server not responding");
            println!("server not responding: {:?}", e.to_string());
            return Err(e);
        }
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    let customer_to_business_response_data = response
                        .json::<CustomerToBusinessPaymentResponseData>()
                        .await?;

                    let my_output = (
                        customer_to_business_response_data,
                        customer_to_business_error_response_data,
                    );

                    return Ok(my_output);
                    //let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    /*
                    let k = String::from(""); //Default value.
                    let m: u32 = 0; //Default value.

                    let my_output = response
                        .json::<CustomerToBusinessPaymentResponseData>()
                        .await?;

                    let merchant_request_id = &my_output.MerchantRequestID.as_ref().unwrap_or(&k);
                    let checkout_request_id = &my_output.CheckoutRequestID.as_ref().unwrap_or(&k);
                    let response_code = &my_output.ResponseCode.as_ref().unwrap_or(&k);
                    let response_description =
                        &my_output.ResponseDescription.as_ref().unwrap_or(&k);
                    let customer_message = &my_output.CustomerMessage.as_ref().unwrap_or(&k);

                    //
                    customer_to_business_response_data.MerchantRequestID =
                        Some(merchant_request_id.to_string());
                    customer_to_business_response_data.CheckoutRequestID =
                        Some(checkout_request_id.to_string());
                    customer_to_business_response_data.ResponseCode =
                        Some(response_code.to_string());
                    customer_to_business_response_data.ResponseDescription =
                        Some(response_description.to_string());
                    customer_to_business_response_data.CustomerMessage =
                        Some(customer_message.to_string());
                    */
                    /*
                    println!(
                        "business_to_customer_response_data: {:?}",
                        &business_to_customer_response_data
                    );
                    */

                    /*
                    create_b2c_acknowledgement(
                        &data,
                        originator_conversation_id.to_string(),
                        conversation_id.to_string(),
                        response_code.to_string(),
                        response_description.to_string(),
                        business_to_customer_data.CommandID,
                        business_to_customer_data.PartyA,
                        business_to_customer_data.PartyB,
                        business_to_customer_data.Amount,
                        request_id,
                        error_code,
                        error_message,
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
                s => {
                    let customer_to_business_error_response_data = response
                        .json::<CustomerToBusinessPaymentErrorResponseData>()
                        .await?;

                    let my_output = (
                        customer_to_business_response_data,
                        customer_to_business_error_response_data,
                    );

                    return Ok(my_output);

                    //let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    /*
                    let k = String::from(""); //Default value.
                    let m: u32 = 0; //Default value.
                    let my_output = response
                        .json::<CustomerToBusinessPaymentErrorResponseData>()
                        .await?;

                    let request_id = &my_output.requestId.as_ref().unwrap_or(&k);
                    let error_code = &my_output.errorCode.as_ref().unwrap_or(&k);
                    let error_message = &my_output.errorMessage.as_ref().unwrap_or(&k);

                    let merchant_request_id = request_id.to_string();
                    let checkout_request_id = String::from("");
                    let response_code = error_code.to_string();
                    let response_description = error_message.to_string();
                    let customer_message = String::from("");

                    customer_to_business_response_data.MerchantRequestID =
                        Some(merchant_request_id);
                    customer_to_business_response_data.CheckoutRequestID =
                        Some(checkout_request_id);
                    customer_to_business_response_data.ResponseCode = Some(response_code);
                    customer_to_business_response_data.ResponseDescription =
                        Some(response_description);
                    customer_to_business_response_data.CustomerMessage = Some(customer_message);
                    */

                    //println!("my_output: {:?}", &my_output);

                    /*
                    create_b2c_acknowledgement(
                        &data,
                        originator_conversation_id.to_string(),
                        conversation_id.to_string(),
                        response_code.to_string(),
                        response_description.to_string(),
                        business_to_customer_data.CommandID,
                        business_to_customer_data.PartyA,
                        business_to_customer_data.PartyB,
                        business_to_customer_data.Amount,
                        request_id.to_string(),
                        error_code.to_string(),
                        error_message.to_string(),
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
            }
        }
    };

    let my_output = (
        customer_to_business_response_data,
        customer_to_business_error_response_data,
    );

    Ok(my_output)
}

pub async fn business_paybill(
    business_paybill_details: BusinessPayBillInputDetails,
    access_token: String,
) -> std::result::Result<
    (
        BusinessPayBillResponseData,
        BusinessPayBillErrorResponseData,
    ),
    reqwest::Error,
> {
    let api_url: String = business_paybill_details.get_api_url();
    let _initiator: String = business_paybill_details.get_initiator();
    let security_credential: String = business_paybill_details.get_security_credential();
    let command_id: String = business_paybill_details.get_command_id();
    let sender_identifier_type: String = business_paybill_details.get_sender_identifier_type();
    let reciever_identifier_type: String = business_paybill_details.get_reciever_identifier_type();
    let _amount: u32 = business_paybill_details.get_amount();
    let party_a: String = business_paybill_details.get_party_a();
    let party_b: String = business_paybill_details.get_party_b();
    let account_reference: String = business_paybill_details.get_account_reference();
    let _requester: String = business_paybill_details.get_requester();
    let _remarks: String = business_paybill_details.get_remarks();
    let queue_time_out_url: String = business_paybill_details.get_queue_time_out_url();
    let result_url: String = business_paybill_details.get_result_url();
    /*
    let business_paybill_data = BusinessPayBillData {
        Initiator: _initiator,
        SecurityCredential: security_credential,
        CommandID: command_id,
        SenderIdentifierType: sender_identifier_type,
        RecieverIdentifierType: reciever_identifier_type,
        Amount: _amount,
        PartyA: party_a,
        PartyB: party_b,
        AccountReference: account_reference,
        Requester: _requester,
        Remarks: _remarks,
        QueueTimeOutURL: queue_time_out_url,
        ResultURL: result_url,
    };
    */
    let business_paybill_data = build_business_paybill_data(
        _initiator,
        security_credential,
        command_id,
        sender_identifier_type,
        reciever_identifier_type,
        _amount,
        party_a,
        party_b,
        account_reference,
        _requester,
        _remarks,
        queue_time_out_url,
        result_url,
    );
    let business_paybill_response_data =
        build_business_paybill_response_data(None, None, None, None);

    let business_paybill_error_response_data =
        build_business_paybill_error_response_data(None, None, None);

    /*
    println!("access_token: {:?}", &access_token);
    println!("api_url: {:?}", &api_url);
    println!(
        "customer_to_business_data: {:?}",
        &customer_to_business_data
    );
    */
    let client = reqwest::Client::new();
    // "%Y-%m-%d %H:%M:%S" i.e "yyyy-MM-dd HH:mm:ss"
    // "%Y-%m-%d %H:%M:%S%.3f" i.e "yyyy-MM-dd HH:mm:ss.SSS"
    //let date_to_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let res = client
        .post(api_url)
        /*
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", access_token)
        */
        .headers(build_headers(access_token))
        .json(&business_paybill_data)
        .send()
        //.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
        .await;

    match res {
        Err(e) => {
            //println!("server not responding");
            println!("server not responding: {:?}", e.to_string());
            return Err(e);
        }
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    let business_paybill_response_data =
                        response.json::<BusinessPayBillResponseData>().await?;

                    let my_output = (
                        business_paybill_response_data,
                        business_paybill_error_response_data,
                    );

                    return Ok(my_output);
                    /*
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.
                    let m: u32 = 0; //Default value.

                    let my_output = response.json::<BusinessPayBillResponseData>().await?;

                    let originator_conversation_id =
                        &my_output.OriginatorConversationID.as_ref().unwrap_or(&k);
                    let conversation_id = &my_output.ConversationID.as_ref().unwrap_or(&k);
                    let response_code = &my_output.ResponseCode.as_ref().unwrap_or(&k);
                    let response_description =
                        &my_output.ResponseDescription.as_ref().unwrap_or(&k);

                    //
                    business_paybill_response_data.OriginatorConversationID =
                        Some(originator_conversation_id.to_string());
                    business_paybill_response_data.ConversationID =
                        Some(conversation_id.to_string());
                    business_paybill_response_data.ResponseCode = Some(response_code.to_string());
                    business_paybill_response_data.ResponseDescription =
                        Some(response_description.to_string());
                    */
                    /*
                    println!(
                        "business_to_customer_response_data: {:?}",
                        &business_to_customer_response_data
                    );
                    */

                    /*
                    create_b2c_acknowledgement(
                        &data,
                        originator_conversation_id.to_string(),
                        conversation_id.to_string(),
                        response_code.to_string(),
                        response_description.to_string(),
                        business_to_customer_data.CommandID,
                        business_to_customer_data.PartyA,
                        business_to_customer_data.PartyB,
                        business_to_customer_data.Amount,
                        request_id,
                        error_code,
                        error_message,
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
                s => {
                    let business_paybill_error_response_data =
                        response.json::<BusinessPayBillErrorResponseData>().await?;

                    let my_output = (
                        business_paybill_response_data,
                        business_paybill_error_response_data,
                    );

                    return Ok(my_output);
                    /*
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.
                    let m: u32 = 0; //Default value.
                    let my_output = response.json::<BusinessPayBillErrorResponseData>().await?;

                    let request_id = &my_output.requestId.as_ref().unwrap_or(&k);
                    let error_code = &my_output.errorCode.as_ref().unwrap_or(&k);
                    let error_message = &my_output.errorMessage.as_ref().unwrap_or(&k);

                    let originator_conversation_id = request_id.to_string();
                    let conversation_id = String::from("");
                    let response_code = error_code.to_string();
                    let response_description = error_message.to_string();

                    business_paybill_response_data.OriginatorConversationID =
                        Some(originator_conversation_id.to_string());
                    business_paybill_response_data.ConversationID =
                        Some(conversation_id.to_string());
                    business_paybill_response_data.ResponseCode = Some(response_code.to_string());
                    business_paybill_response_data.ResponseDescription =
                        Some(response_description.to_string());
                    */

                    //println!("my_output: {:?}", &my_output);

                    /*
                    create_b2c_acknowledgement(
                        &data,
                        originator_conversation_id.to_string(),
                        conversation_id.to_string(),
                        response_code.to_string(),
                        response_description.to_string(),
                        business_to_customer_data.CommandID,
                        business_to_customer_data.PartyA,
                        business_to_customer_data.PartyB,
                        business_to_customer_data.Amount,
                        request_id.to_string(),
                        error_code.to_string(),
                        error_message.to_string(),
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
            }
        }
    };

    let my_output = (
        business_paybill_response_data,
        business_paybill_error_response_data,
    );

    Ok(my_output)
}

async fn business_buy_goods(
    business_buy_goods_details: BusinessBuyGoodsInputDetails,
    access_token: String,
) -> std::result::Result<
    (
        BusinessBuyGoodsResponseData,
        BusinessBuyGoodsErrorResponseData,
    ),
    reqwest::Error,
> {
    let api_url: String = business_buy_goods_details.get_api_url();
    let _initiator: String = business_buy_goods_details.get_initiator();
    let security_credential: String = business_buy_goods_details.get_security_credential();
    let command_id: String = business_buy_goods_details.get_command_id();
    let sender_identifier_type: String = business_buy_goods_details.get_sender_identifier_type();
    let reciever_identifier_type: String =
        business_buy_goods_details.get_reciever_identifier_type();
    let _amount: u32 = business_buy_goods_details.get_amount();
    let party_a: String = business_buy_goods_details.get_party_a();
    let party_b: String = business_buy_goods_details.get_party_b();
    let account_reference: String = business_buy_goods_details.get_account_reference();
    let _requester: String = business_buy_goods_details.get_requester();
    let _remarks: String = business_buy_goods_details.get_remarks();
    let queue_time_out_url: String = business_buy_goods_details.get_queue_time_out_url();
    let result_url: String = business_buy_goods_details.get_result_url();
    /*
    let business_buy_goods_data = BusinessBuyGoodsData {
        Initiator: _initiator,
        SecurityCredential: security_credential,
        CommandID: command_id,
        SenderIdentifierType: sender_identifier_type,
        RecieverIdentifierType: reciever_identifier_type,
        Amount: _amount,
        PartyA: party_a,
        PartyB: party_b,
        AccountReference: account_reference,
        Requester: _requester,
        Remarks: _remarks,
        QueueTimeOutURL: queue_time_out_url,
        ResultURL: result_url,
    };
    */
    let business_buy_goods_data = build_business_buy_goods_data(
        _initiator,
        security_credential,
        command_id,
        sender_identifier_type,
        reciever_identifier_type,
        _amount,
        party_a,
        party_b,
        account_reference,
        _requester,
        _remarks,
        queue_time_out_url,
        result_url,
    );
    let business_buy_goods_response_data =
        build_business_buy_goods_response_data(None, None, None, None);

    let business_buy_goods_error_response_data =
        build_business_buy_goods_error_response_data(None, None, None);

    /*
    println!("access_token: {:?}", &access_token);
    println!("api_url: {:?}", &api_url);
    println!(
        "customer_to_business_data: {:?}",
        &customer_to_business_data
    );
    */
    let client = reqwest::Client::new();
    // "%Y-%m-%d %H:%M:%S" i.e "yyyy-MM-dd HH:mm:ss"
    // "%Y-%m-%d %H:%M:%S%.3f" i.e "yyyy-MM-dd HH:mm:ss.SSS"
    //let date_to_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let res = client
        .post(api_url)
        /*
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", access_token)
        */
        .headers(build_headers(access_token))
        .json(&business_buy_goods_data)
        .send()
        //.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
        .await;

    match res {
        Err(e) => {
            //println!("server not responding");
            println!("server not responding: {:?}", e.to_string());
            return Err(e);
        }
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    let business_buy_goods_response_data =
                        response.json::<BusinessBuyGoodsResponseData>().await?;

                    let my_output = (
                        business_buy_goods_response_data,
                        business_buy_goods_error_response_data,
                    );

                    return Ok(my_output);
                    /*
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.
                    let m: u32 = 0; //Default value.

                    let my_output = response.json::<BusinessBuyGoodsResponseData>().await?;

                    let originator_conversation_id =
                        &my_output.OriginatorConversationID.as_ref().unwrap_or(&k);
                    let conversation_id = &my_output.ConversationID.as_ref().unwrap_or(&k);
                    let response_code = &my_output.ResponseCode.as_ref().unwrap_or(&k);
                    let response_description =
                        &my_output.ResponseDescription.as_ref().unwrap_or(&k);

                    //
                    business_buy_goods_response_data.OriginatorConversationID =
                        Some(originator_conversation_id.to_string());
                    business_buy_goods_response_data.ConversationID =
                        Some(conversation_id.to_string());
                    business_buy_goods_response_data.ResponseCode = Some(response_code.to_string());
                    business_buy_goods_response_data.ResponseDescription =
                        Some(response_description.to_string());
                    */
                    /*
                    println!(
                        "business_to_customer_response_data: {:?}",
                        &business_to_customer_response_data
                    );
                    */

                    /*
                    create_b2c_acknowledgement(
                        &data,
                        originator_conversation_id.to_string(),
                        conversation_id.to_string(),
                        response_code.to_string(),
                        response_description.to_string(),
                        business_to_customer_data.CommandID,
                        business_to_customer_data.PartyA,
                        business_to_customer_data.PartyB,
                        business_to_customer_data.Amount,
                        request_id,
                        error_code,
                        error_message,
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
                s => {
                    let business_buy_goods_error_response_data =
                        response.json::<BusinessBuyGoodsErrorResponseData>().await?;

                    let my_output = (
                        business_buy_goods_response_data,
                        business_buy_goods_error_response_data,
                    );

                    return Ok(my_output);
                    /*
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.
                    let m: u32 = 0; //Default value.
                    let my_output = response.json::<BusinessBuyGoodsErrorResponseData>().await?;

                    let request_id = &my_output.requestId.as_ref().unwrap_or(&k);
                    let error_code = &my_output.errorCode.as_ref().unwrap_or(&k);
                    let error_message = &my_output.errorMessage.as_ref().unwrap_or(&k);

                    let originator_conversation_id = request_id.to_string();
                    let conversation_id = String::from("");
                    let response_code = error_code.to_string();
                    let response_description = error_message.to_string();

                    business_buy_goods_response_data.OriginatorConversationID =
                        Some(originator_conversation_id.to_string());
                    business_buy_goods_response_data.ConversationID =
                        Some(conversation_id.to_string());
                    business_buy_goods_response_data.ResponseCode = Some(response_code.to_string());
                    business_buy_goods_response_data.ResponseDescription =
                        Some(response_description.to_string());
                    */

                    //println!("my_output: {:?}", &my_output);

                    /*
                    create_b2c_acknowledgement(
                        &data,
                        originator_conversation_id.to_string(),
                        conversation_id.to_string(),
                        response_code.to_string(),
                        response_description.to_string(),
                        business_to_customer_data.CommandID,
                        business_to_customer_data.PartyA,
                        business_to_customer_data.PartyB,
                        business_to_customer_data.Amount,
                        request_id.to_string(),
                        error_code.to_string(),
                        error_message.to_string(),
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                    */
                }
            }
        }
    };

    let my_output = (
        business_buy_goods_response_data,
        business_buy_goods_error_response_data,
    );

    Ok(my_output)
}
