<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Anulação Liquidações e Substituições</name>
   <tag></tag>
   <elementGuidId>830362ae-7dc1-4d6a-9ae4-00151dfa395a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic ZXh0ZXJuYWxBcGk6ZiRjRSpzUnY=</value>
      <webElementGuid>b65ca835-1389-4e68-8449-72b337065d9a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:ws=&quot;http://tistech.co.ao/ws&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;ws:cancelPayTenderRequest>
         &lt;ws:idEmissora>&lt;/ws:idEmissora>
         &lt;ws:idNIF>&lt;/ws:idNIF>
         &lt;ws:idLiqAnular>&lt;/ws:idLiqAnular>
      &lt;/ws:cancelPayTenderRequest>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://10.129.105.122:7021/external-liquidation-api/ws/soap/cancelPayTender.wsdl</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress>http://10.129.105.122:7021/external-liquidation-api/ws/soap/cancelPayTender.wsdl</wsdlAddress>
</WebServiceRequestEntity>
