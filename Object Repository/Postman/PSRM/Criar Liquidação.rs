<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Criar Liquidação</name>
   <tag></tag>
   <elementGuidId>399260cd-1ef2-4749-8afa-94d8040c1f04</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003csoapenv:Envelope xmlns:cm\u003d\&quot;http://ouaf.oracle.com/webservices/cm/CM-CreatePaySigfeeAsycuda\&quot; xmlns:soapenv\u003d\&quot;http://schemas.xmlsoap.org/soap/envelope/\&quot;\u003e\n\t\u003csoapenv:Header\u003e\n      \u003cwsse:Security soapenv:mustUnderstand\u003d\&quot;1\&quot; xmlns:wsse\u003d\&quot;http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd\&quot; xmlns:wsu\u003d\&quot;http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd\&quot;\u003e\n         \u003cwsu:Timestamp wsu:Id\u003d\&quot;TS-3B529398C38830E72016488368943474\&quot;\u003e\n            \u003cwsu:Created\u003e2022-04-01T18:14:54.346Z\u003c/wsu:Created\u003e\n            \u003cwsu:Expires\u003e2022-04-01T19:54:54.346Z\u003c/wsu:Expires\u003e\n         \u003c/wsu:Timestamp\u003e\n         \u003cwsse:UsernameToken wsu:Id\u003d\&quot;UsernameToken-3B529398C38830E72016488368890653\&quot;\u003e\n            \u003cwsse:Username\u003ePUSER10\u003c/wsse:Username\u003e\n            \u003cwsse:Password Type\u003d\&quot;http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordText\&quot;\u003epassword10\u003c/wsse:Password\u003e\n            \u003cwsse:Nonce EncodingType\u003d\&quot;http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-soap-message-security-1.0#Base64Binary\&quot;\u003eFlxPg2W/SCXuSDooEeLKUw\u003d\u003d\u003c/wsse:Nonce\u003e\n            \u003cwsu:Created\u003e2022-04-01T18:14:49.065Z\u003c/wsu:Created\u003e\n         \u003c/wsse:UsernameToken\u003e\n      \u003c/wsse:Security\u003e\n\t\u003c/soapenv:Header\u003e\n\t\u003csoapenv:Body\u003e\n\t\t\u003ccm:CM-CreatePaySigfeeAsycuda\u003e\n\t\t\t\u003c!--Zero or more repetitions:--\u003e\n\t\t\t\u003ccm:cmTaxList\u003e\n\t\t\t\t\u003c!--Zero or more repetitions:--\u003e\n\t\t\t\t\u003ccm:cmNodeList\u003e\n\t\t\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\t\t\u003ccm:cmTaxType\u003eD15\u003c/cm:cmTaxType\u003e\n\t\t\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\t\t\u003ccm:cmSigttaxValue\u003e50000000\u003c/cm:cmSigttaxValue\u003e\n\t\t\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\t\t\u003ccm:cmOtherRevTaxpNif/\u003e\n\t\t\t\t\u003c/cm:cmNodeList\u003e\n\t\t\t\u003c/cm:cmTaxList\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:receiveDate\u003e2021-03-24\u003c/cm:receiveDate\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:cmDcTaxpNif\u003e100909092291\u003c/cm:cmDcTaxpNif\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:cmDocument/\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:personId/\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:cmDcAccount/\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:cmCollectionFlg\u003eVOL\u003c/cm:cmCollectionFlg\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:cmAssessmentFormFlg\u003eSASS\u003c/cm:cmAssessmentFormFlg\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:cmAssessmentTypeFlg\u003eDEFI\u003c/cm:cmAssessmentTypeFlg\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:cmFrequencyFlg\u003eMON\u003c/cm:cmFrequencyFlg\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:cmIssueDate\u003e2022-03-24\u003c/cm:cmIssueDate\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:cmDueDate\u003e2022-04-29\u003c/cm:cmDueDate\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:cmPaymentDate\u003e2022-05-31\u003c/cm:cmPaymentDate\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:cmPeriodFlg\u003eM03\u003c/cm:cmPeriodFlg\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:cmYear\u003e2022\u003c/cm:cmYear\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:intSourceFlg\u003eASYC\u003c/cm:intSourceFlg\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:rsltResponse/\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:darNumber/\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:dliNumber/\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:cmAsyRefId\u003ePJ14SLP_MAR22_0044\u003c/cm:cmAsyRefId\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:cmCancelFlag/\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:externalUserId/\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:cmTaxOffice\u003e1156\u003c/cm:cmTaxOffice\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:documentLocator/\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:cmEmisEntity/\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003ccm:cmyesNoFlag/\u003e\n\t\t\u003c/cm:CM-CreatePaySigfeeAsycuda\u003e\n\t\u003c/soapenv:Body\u003e\n\u003c/soapenv:Envelope\u003e&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic UFVTRVIxMDpwYXNzd29yZDEw</value>
      <webElementGuid>4cf2f12a-8918-4715-8669-e0f1f633aa1c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Date</name>
      <type>Main</type>
      <value></value>
      <webElementGuid>59818ec8-79ff-4dad-8c21-7a684af2073e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://10.129.105.51:7004/ouaf/webservices/CM-CreatePaySigfeeAsycuda</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
