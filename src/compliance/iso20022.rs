use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TxDetails {
    pub tx_id: String,
    pub amount: String,
    pub currency: String,
    pub debtor: String, // Sender
    pub creditor: String, // Receiver
}

/**
 * Generates a pacs.008.001.08 XML message.
 * Note: Minimum Variable Product (MVP) implementation using manual string formatting
 * to avoid adding heavy XML dependencies to the existing environment.
 */
pub fn generate_pacs008(details: &TxDetails) -> String {
    format!(
r#"<?xml version="1.0" encoding="UTF-8"?>
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">
    <FIToFICstmrCdtTrf>
        <GrpHdr>
            <MsgId>{msg_id}</MsgId>
            <CreDtTm>{timestamp}</CreDtTm>
            <NbOfTxs>1</NbOfTxs>
            <SttlmInf>
                <SttlmMtd>CLRG</SttlmMtd>
            </SttlmInf>
        </GrpHdr>
        <CdtTrfTxInf>
            <PmtId>
                <EndToEndId>{tx_id}</EndToEndId>
                <TxId>{tx_id}</TxId>
            </PmtId>
            <IntrBkSttlmAmt Ccy="{currency}">{amount}</IntrBkSttlmAmt>
            <Dbtr>
                <Nm>{debtor}</Nm>
            </Dbtr>
            <Cdtr>
                <Nm>{creditor}</Nm>
            </Cdtr>
        </CdtTrfTxInf>
    </FIToFICstmrCdtTrf>
</Document>"#,
        msg_id = format!("MSG-{}", details.tx_id),
        timestamp = chrono::Utc::now().to_rfc3339(),
        tx_id = details.tx_id,
        currency = details.currency,
        amount = details.amount,
        debtor = details.debtor,
        creditor = details.creditor
    )
}
