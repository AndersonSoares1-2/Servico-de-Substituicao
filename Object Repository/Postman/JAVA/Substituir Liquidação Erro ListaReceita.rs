<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Substituir Liquidação Erro ListaReceita</name>
   <tag></tag>
   <elementGuidId>07071ea8-2abc-4276-8b0e-bd90e1d62d7f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003csoapenv:Envelope xmlns:soapenv\u003d\&quot;http://schemas.xmlsoap.org/soap/envelope/\&quot; xmlns:ws\u003d\&quot;http://tistech.co.ao/ws\&quot;\u003e\n\t\u003csoapenv:Header/\u003e\n\t\u003csoapenv:Body\u003e\n\t\t\u003cws:replacePaymentRequest\u003e\n\t\t\t\u003cws:idEmissora\u003e03\u003c/ws:idEmissora\u003e\n\t\t\t\u003cws:idExterno\u003e13072022JS4\u003c/ws:idExterno\u003e\n\t\t\t\u003cws:idNif\u003e5000178772\u003c/ws:idNif\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003cws:idConta/\u003e\n\t\t\t\u003cws:idEntLiq\u003e1156\u003c/ws:idEntLiq\u003e\n\t\t\t\u003cws:dataLiq\u003e2022-03-24\u003c/ws:dataLiq\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003cws:tipoCobranca\u003eVOL\u003c/ws:tipoCobranca\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003cws:formaLiquidacao\u003eSASS\u003c/ws:formaLiquidacao\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003cws:tipoLiquidacao\u003eDEFI\u003c/ws:tipoLiquidacao\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003cws:anoExercicio\u003e2022\u003c/ws:anoExercicio\u003e\n\t\t\t\u003c!--Optional:--\u003e\n\t\t\t\u003cws:perExercicio\u003eM03\u003c/ws:perExercicio\u003e\n\t\t\t\u003cws:dataLimPag\u003e2022-04-29\u003c/ws:dataLimPag\u003e\n\t\t\t\u003c!--1 or more repetitions:--\u003e\n\t\t\t\u003cws:listaReceita\u003e\n\t\t\t\t\u003cws:codReceita\u003eD15\u003c/ws:codReceita\u003e\n\t\t\t\t\u003cws:idNifEntidade/\u003e\n\t\t\t\t\u003cws:valorLiq\u003e56789.54\u003c/ws:valorLiq\u003e\n\t\t\t\t\u003cws:cdProvincia\u003eLA\u003c/ws:cdProvincia\u003e\n\t\t\t\t\u003cws:cdMunicipio\u003eLUANDA\u003c/ws:cdMunicipio\u003e\n\t\t\t\u003c/ws:listaReceita\u003e\n\t\t\t\u003cws:dataPagEscr\u003e2022-04-29\u003c/ws:dataPagEscr\u003e\n\t\t\t\u003cws:idLiqAnterior\u003e13072022JS3\u003c/ws:idLiqAnterior\u003e\n\t\t\u003c/ws:replacePaymentRequest\u003e\n\t\u003c/soapenv:Body\u003e\n\u003c/soapenv:Envelope\u003e&quot;,
  &quot;contentType&quot;: &quot;application/xml&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic ZXh0ZXJuYWxBcGk6ZiRjRSpzUnY=</value>
      <webElementGuid>6b7417d9-63e4-4ece-9c3f-3004ebbb9406</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://10.129.105.122:7021/external-liquidation-api/ws/soap/replacePayment.wsdl</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
