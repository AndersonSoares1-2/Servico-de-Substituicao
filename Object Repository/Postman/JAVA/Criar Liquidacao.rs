<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Criar Liquidacao</name>
   <tag></tag>
   <elementGuidId>db8e0b57-f155-4ef9-b9b6-c73211f470ee</elementGuidId>
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
      <webElementGuid>bcde6aaf-e936-466d-8fa6-90e2f5a74703</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:ws=&quot;http://tistech.co.ao/ws&quot;>
   &lt;soapenv:Header>
   &lt;/soapenv:Header>
   &lt;soapenv:Body>
      &lt;ws:createPayGenericRequest>
         &lt;ws:idEmissora>03&lt;/ws:idEmissora>
         &lt;ws:idExterno>CREATE01999121&lt;/ws:idExterno>
         &lt;ws:idNif>5000187534&lt;/ws:idNif>
         &lt;!--Optional:-->
         &lt;ws:idConta>&lt;/ws:idConta>
         &lt;ws:idEntLiq>1156&lt;/ws:idEntLiq>
         &lt;ws:dataLiq>2021-03-24&lt;/ws:dataLiq>
         &lt;!--Optional:-->
         &lt;ws:tipoCobranca>VOL&lt;/ws:tipoCobranca>
         &lt;!--Optional:-->
         &lt;ws:formaLiquidacao>SASS&lt;/ws:formaLiquidacao>
         &lt;!--Optional:-->
         &lt;ws:tipoLiquidacao>DEFI&lt;/ws:tipoLiquidacao>
         &lt;!--Optional:-->
         &lt;ws:anoExercicio>2021&lt;/ws:anoExercicio>
         &lt;!--Optional:-->
         &lt;ws:perExercicio>M04&lt;/ws:perExercicio>
         &lt;ws:dataLimPag>2021-04-29&lt;/ws:dataLimPag>
         &lt;!--1 or more repetitions:-->
         &lt;ws:listaReceita>
            &lt;ws:codReceita>A28&lt;/ws:codReceita>
            &lt;!--Optional:-->
            &lt;ws:idNifEntidade>5000178900&lt;/ws:idNifEntidade>
            &lt;ws:valorLiq>789654.88&lt;/ws:valorLiq>
            &lt;!--Optional:-->
            &lt;ws:cdProvincia>LA&lt;/ws:cdProvincia>
            &lt;!--Optional:-->
            &lt;ws:cdMunicipio>LUANDA&lt;/ws:cdMunicipio>
         &lt;/ws:listaReceita>
         &lt;ws:indPagEscr>F&lt;/ws:indPagEscr>
         &lt;!--Optional:-->
         &lt;ws:dataPagEscr>&lt;/ws:dataPagEscr>
         &lt;!--Optional:-->
         &lt;ws:documentoAssociado>&lt;/ws:documentoAssociado>
      &lt;/ws:createPayGenericRequest>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://10.129.105.122:7021/external-liquidation-api/ws/soap/settlementPayGeneric.wsdl</soapServiceEndpoint>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

//WS.verifyElementText(response, 'createPayGenericResponse.idExterno', 'CREATE01999121')

//WS.verifyElementPropertyValue(response, 'createPayGenericResponse.idrsltResponse', 'ID de Referência Asycuda / SIGFE (CREATE01999121) já existe.')
</verificationScript>
   <wsdlAddress>http://10.129.105.122:7021/external-liquidation-api/ws/soap/settlementPayGeneric.wsdl</wsdlAddress>
</WebServiceRequestEntity>
