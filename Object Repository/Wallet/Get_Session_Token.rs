<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get_Session_Token</name>
   <tag></tag>
   <elementGuidId>f5c53b73-b0b1-4f21-8ef8-d2cabef77341</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>X-Genesis-PartnerToken</name>
      <type>Main</type>
      <value>${partner}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>X-Genesis-Secret</name>
      <type>Main</type>
      <value>${secretkey}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://${url}/m4/wallet/balance/${player_id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'krug-gw-colo.star9ad.com'</defaultValue>
      <description></description>
      <id>5d243f24-169e-4a0c-b172-7c0f24ad8425</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>findTestData('NG Game Release Acceptance Test').getValue(2, 1)</defaultValue>
      <description></description>
      <id>f6c5ad29-4d99-41d2-b17e-a8fdfb1fdd64</id>
      <masked>false</masked>
      <name>partner</name>
   </variables>
   <variables>
      <defaultValue>findTestData('NG Game Release Acceptance Test').getValue(3, 1)</defaultValue>
      <description></description>
      <id>fddf52b3-f2fb-4c1e-9b9a-d102c8badab7</id>
      <masked>false</masked>
      <name>secretkey</name>
   </variables>
   <variables>
      <defaultValue>findTestData('NG Game Release Acceptance Test').getValue(5, 1)</defaultValue>
      <description></description>
      <id>4673a9c8-2fd2-483b-962b-61d307e48dab</id>
      <masked>false</masked>
      <name>player_id</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager


RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)
assertThat(response.getResponseText()).contains('session_token')

String newline = System.getProperty(&quot;line.separator&quot;)
def getsession = new groovy.json.JsonSlurper()
def result_getsession = getsession.parseText(response.getResponseBodyContent())
result = response.getResponseBodyContent()
println(result)

def session_token = result_getsession.session_token
println (&quot;**** Session Token is **** :&quot;+newline+&quot;**** &quot;+session_token+&quot; ****&quot;)
GlobalVariable.session_token = session_token
println (&quot;**** Session Token is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.session_token+&quot; ****&quot;)

def wallet_balance = result_getsession.internal_balance
println (&quot;**** Wallet Balance is **** :&quot;+newline+&quot;**** &quot;+wallet_balance+&quot; ****&quot;)
GlobalVariable.wallet_balance = wallet_balance
println (&quot;**** Wallet Balance is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.wallet_balance+&quot; ****&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
