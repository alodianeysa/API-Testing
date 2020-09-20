<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_List_User</name>
   <tag></tag>
   <elementGuidId>739590fb-1940-4935-9c99-7250b7afe0a2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;page\&quot;: 2,\n    \&quot;per_page\&quot;: 6,\n    \&quot;total\&quot;: 12,\n    \&quot;total_pages\&quot;: 2,\n    \&quot;data\&quot;: [\n        {\n            \&quot;id\&quot;: 7,\n            \&quot;email\&quot;: \&quot;michael.lawson@reqres.in\&quot;,\n            \&quot;first_name\&quot;: \&quot;Michael\&quot;,\n            \&quot;last_name\&quot;: \&quot;Lawson\&quot;,\n            \&quot;avatar\&quot;: \&quot;https://s3.amazonaws.com/uifaces/faces/twitter/follettkyle/128.jpg\&quot;\n        },\n        {\n            \&quot;id\&quot;: 8,\n            \&quot;email\&quot;: \&quot;lindsay.ferguson@reqres.in\&quot;,\n            \&quot;first_name\&quot;: \&quot;Lindsay\&quot;,\n            \&quot;last_name\&quot;: \&quot;Ferguson\&quot;,\n            \&quot;avatar\&quot;: \&quot;https://s3.amazonaws.com/uifaces/faces/twitter/araa3185/128.jpg\&quot;\n        },\n        {\n            \&quot;id\&quot;: 9,\n            \&quot;email\&quot;: \&quot;tobias.funke@reqres.in\&quot;,\n            \&quot;first_name\&quot;: \&quot;Tobias\&quot;,\n            \&quot;last_name\&quot;: \&quot;Funke\&quot;,\n            \&quot;avatar\&quot;: \&quot;https://s3.amazonaws.com/uifaces/faces/twitter/vivekprvr/128.jpg\&quot;\n        },\n        {\n            \&quot;id\&quot;: 10,\n            \&quot;email\&quot;: \&quot;byron.fields@reqres.in\&quot;,\n            \&quot;first_name\&quot;: \&quot;Byron\&quot;,\n            \&quot;last_name\&quot;: \&quot;Fields\&quot;,\n            \&quot;avatar\&quot;: \&quot;https://s3.amazonaws.com/uifaces/faces/twitter/russoedu/128.jpg\&quot;\n        },\n        {\n            \&quot;id\&quot;: 11,\n            \&quot;email\&quot;: \&quot;george.edwards@reqres.in\&quot;,\n            \&quot;first_name\&quot;: \&quot;George\&quot;,\n            \&quot;last_name\&quot;: \&quot;Edwards\&quot;,\n            \&quot;avatar\&quot;: \&quot;https://s3.amazonaws.com/uifaces/faces/twitter/mrmoiree/128.jpg\&quot;\n        },\n        {\n            \&quot;id\&quot;: 12,\n            \&quot;email\&quot;: \&quot;rachel.howell@reqres.in\&quot;,\n            \&quot;first_name\&quot;: \&quot;Rachel\&quot;,\n            \&quot;last_name\&quot;: \&quot;Howell\&quot;,\n            \&quot;avatar\&quot;: \&quot;https://s3.amazonaws.com/uifaces/faces/twitter/hebertialmeida/128.jpg\&quot;\n        }\n    ],\n    \&quot;ad\&quot;: {\n        \&quot;company\&quot;: \&quot;StatusCode Weekly\&quot;,\n        \&quot;url\&quot;: \&quot;http://statuscode.org/\&quot;,\n        \&quot;text\&quot;: \&quot;A weekly newsletter focusing on software development, infrastructure, the server, performance, and the stack end of things.\&quot;\n    }\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://reqres.in/api/users?page=2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyElementPropertyValue(response, 'data[0].first_name', &quot;Michael&quot;)

WS.verifyElementPropertyValue(response, 'data[0].last_name', &quot;Lawson&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
