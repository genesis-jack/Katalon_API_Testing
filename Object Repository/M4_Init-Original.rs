<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>M4_Init-Original</name>
   <tag></tag>
   <elementGuidId>748cd2ef-7f98-4256-98a6-ee8721513a82</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
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
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>X-Genesis-UserId</name>
      <type>Main</type>
      <value>eyJrcnVnX3Nlc3Npb24iOiJiZjA0YWQxNS1hZDJmLTQ0N2UtOTk0YS00ZTYxNjVkZjZlZjAiLCJ1c2VyX2lkIjoiV2FsbGV0UGxheWVyMDUiLCJwYXJ0bmVyX2NvZGUiOiJCQklOIiwiZ2FtZV9rZXkiOiJNNC0wMDA4IiwiY2xpZW50X2lwIjpudWxsLCJtZXJjaGFudF9jb2RlIjoiIiwiZGV2aWNlIjpudWxsLCJjdXJyZW5jeSI6IkNOWSIsImRlbW9fdWlkIjpudWxsLCJwbGF5ZXJfdHlwZSI6IkhPVVNFIiwiYmV0X2Rlbm9tX2RlZmF1bHQiOjB9</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://${partner}.star88ad.com/m4/gameservice/init/RICH_PANDA_M4_RECORDER</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'c304afdf-2f61-6369-c088-924f99e1be1a'</defaultValue>
      <description></description>
      <id>a4cef006-8b8a-416d-bf3c-726aadc42de5</id>
      <masked>false</masked>
      <name>partner</name>
   </variables>
   <variables>
      <defaultValue>'eyJrcnVnX3Nlc3Npb24iOiJkM2M3MTJhZi1hMmM1LTQwOTMtYTIxMC01ZGNmMDk0NTgwYjgiLCJ1c2VyX2lkIjoiV2FsbGV0UGxheWVyMDUiLCJwYXJ0bmVyX2NvZGUiOiJCQklOIiwiZ2FtZV9rZXkiOiJNNC0wMDA4IiwiY2xpZW50X2lwIjpudWxsLCJtZXJjaGFudF9jb2RlIjoiIiwiZGV2aWNlIjpudWxsLCJjdXJyZW5jeSI6IkNOWSIsImRlbW9fdWlkIjpudWxsLCJwbGF5ZXJfdHlwZSI6IkhPVVNFIiwiYmV0X2Rlbm9tX2RlZmF1bHQiOjB9'</defaultValue>
      <description></description>
      <id>f1eb5db2-558b-48de-a774-6ef54cc595b4</id>
      <masked>false</masked>
      <name>user_id</name>
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

def init = new groovy.json.JsonSlurper()
def result_init = init.parseText(response.getResponseBodyContent())
def rgs_session_token = result_init.session_token
println (&quot;...value extracted is :&quot;+rgs_session_token)
GlobalVariable.rgs_session_token = rgs_session_token
println (&quot;RGS Session is :&quot;+GlobalVariable.rgs_session_token)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
