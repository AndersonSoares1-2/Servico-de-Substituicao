<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Substituir Liquidacao 001</name>
   <tag></tag>
   <elementGuidId>8324d397-22b5-474a-8601-10b7febfd60e</elementGuidId>
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
      <webElementGuid>506afa26-0045-480f-a691-73b7e76cfc3e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:ws=&quot;http://tistech.co.ao/ws&quot;>
	&lt;soapenv:Header/>
	&lt;soapenv:Body>
		&lt;ws:replacePaymentRequest>
			&lt;ws:idEmissora>03&lt;/ws:idEmissora>
			&lt;ws:idExterno>PJ14SLP_MAR202000216&lt;/ws:idExterno>
			&lt;ws:idNif>5000178772&lt;/ws:idNif>
			&lt;!--Optional:-->
			&lt;ws:idConta/>
			&lt;ws:idEntLiq>1156&lt;/ws:idEntLiq>
			&lt;ws:dataLiq>2022-03-24&lt;/ws:dataLiq>
			&lt;!--Optional:-->
			&lt;ws:tipoCobranca>VOL&lt;/ws:tipoCobranca>
			&lt;!--Optional:-->
			&lt;ws:formaLiquidacao>SASS&lt;/ws:formaLiquidacao>
			&lt;!--Optional:-->
			&lt;ws:tipoLiquidacao>DEFI&lt;/ws:tipoLiquidacao>
			&lt;!--Optional:-->
			&lt;ws:anoExercicio>2022&lt;/ws:anoExercicio>
			&lt;!--Optional:-->
			&lt;ws:perExercicio>M03&lt;/ws:perExercicio>
			&lt;ws:dataLimPag>2022-04-29&lt;/ws:dataLimPag>
			&lt;!--1 or more repetitions:-->
			&lt;ws:listaReceita>
				&lt;ws:codReceita>A28&lt;/ws:codReceita>
				 &lt;ws:idNifEntidade>5000184250&lt;/ws:idNifEntidade>
				&lt;ws:valorLiq>40000.00&lt;/ws:valorLiq>
				&lt;ws:cdProvincia>LA&lt;/ws:cdProvincia>
				&lt;ws:cdMunicipio/>
			&lt;/ws:listaReceita>
			&lt;ws:dataPagEscr>2022-03-24&lt;/ws:dataPagEscr>
			&lt;ws:idLiqAnterior>CREATE01999121&lt;/ws:idLiqAnterior>
		&lt;/ws:replacePaymentRequest>
	&lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://10.129.105.122:7021/external-liquidation-api/ws/soap/replacePayment.wsdl</soapServiceEndpoint>
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
   <wsdlAddress>http://10.129.105.122:7021/external-liquidation-api/ws/soap/replacePayment.wsdl</wsdlAddress>
</WebServiceRequestEntity>
