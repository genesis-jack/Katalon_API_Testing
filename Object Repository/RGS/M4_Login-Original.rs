<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>M4_Login-Original</name>
   <tag></tag>
   <elementGuidId>b5811946-7f76-4414-8baa-1a72755a3a8f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;login_properties\&quot;: [\n    {\n      \&quot;name\&quot;: \&quot;session_token\&quot;,\n      \&quot;value\&quot;: \&quot;fddb6426b250bd918c36129365ba2a5d\&quot;\n    },\n    {\n      \&quot;name\&quot;: \&quot;game_id\&quot;,\n      \&quot;value\&quot;: \&quot;M4-0008\&quot;\n    }\n  ]\n}&quot;,
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
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>X-Genesis-PartnerToken</name>
      <type>Main</type>
      <value>${partner}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://${partner}.star88ad.com/m4/gameservice/login/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'c304afdf-2f61-6369-c088-924f99e1be1a'</defaultValue>
      <description></description>
      <id>7bf2329c-6382-46c0-9cb9-65d21393e4ab</id>
      <masked>false</masked>
      <name>partner</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

def login = new groovy.json.JsonSlurper()
def result_login = login.parseText(response.getResponseBodyContent())
def user_id = result_login.user_id
println (&quot;...value extracted is :&quot;+user_id)
GlobalVariable.user_id = user_id
println (&quot;User ID is :&quot;+GlobalVariable.user_id)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
