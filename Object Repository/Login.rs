<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Login</name>
   <tag></tag>
   <elementGuidId>f224b7c5-3e07-4849-aa6a-736cf5e8bd51</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;partner_token\&quot;: \&quot;${partner}\&quot;,\n    \&quot;player_token\&quot;: \&quot;${session_token}\&quot;,\n    \&quot;game_code\&quot;: \&quot;NG-0063\&quot;,\n    \&quot;device\&quot;: \&quot;DESKTOP\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
   </httpHeaderProperties>
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
      <name>Content</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://nurgs.star9ad.com/ng/sessions/?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.session_token</defaultValue>
      <description></description>
      <id>f58afccd-2677-4453-80f2-9162dae713f1</id>
      <masked>false</masked>
      <name>session_token</name>
   </variables>
   <variables>
      <defaultValue>'c304afdf-2f61-6369-c088-924f99e1be1a'</defaultValue>
      <description></description>
      <id>469e4f86-9bbc-404c-b32b-4922e3199538</id>
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

def rgssessiontoken = result_login.state.session_token
println (&quot;...value extracted is :&quot;+rgssessiontoken)

GlobalVariable.rgs_session_token = rgssessiontoken
println (&quot;GlobalVariable is :&quot;+GlobalVariable.rgs_session_token)

def statetag = result_login.state.state_tag
println (&quot;...value extracted is :&quot;+statetag)

GlobalVariable.state_tag = statetag
println (&quot;GlobalVariable is :&quot;+GlobalVariable.state_tag)

def playerid = result_login.state.player_id
println (&quot;...value extracted is :&quot;+playerid)

GlobalVariable.player_id = playerid
println (&quot;GlobalVariable is :&quot;+GlobalVariable.player_id)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
