<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>take-turn_Pick - Original</name>
   <tag></tag>
   <elementGuidId>7433b70b-a24e-4092-9d41-90c1bd920a90</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n   \&quot;player_id\&quot;: \&quot;1571102\&quot;,\n   \&quot;partner_code\&quot;: \&quot;BBIN\&quot;,\n   \&quot;choice_id\&quot;: \&quot;FS_20\&quot;,\n   \&quot;game_code\&quot;: \&quot;${game_code}\&quot;,\n   \&quot;action\&quot;: \&quot;PICK\&quot;,\n   \&quot;session_token\&quot;: \&quot;ef4614bc-a782-4a70-aee8-7e0f5323e763\&quot;,\n   \&quot;state_tag\&quot;:\&quot;2E547523\&quot;,\n   \&quot;bet_denom_index\&quot;: 1\n}&quot;,
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
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://nurgs.star9ad.com/ng/take-turn/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.player_id</defaultValue>
      <description></description>
      <id>88345572-0624-4879-9ed5-fba5ed710c60</id>
      <masked>false</masked>
      <name>player_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.partner_code</defaultValue>
      <description></description>
      <id>5d4093f1-b3be-4438-9382-dae44691ce15</id>
      <masked>false</masked>
      <name>partner_code</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.rgs_session_token</defaultValue>
      <description></description>
      <id>edf13d5c-239b-4355-803f-4b21a4176c2e</id>
      <masked>false</masked>
      <name>rgs_session_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.state_tag</defaultValue>
      <description></description>
      <id>81775165-c804-481b-873f-5ed32c2334d0</id>
      <masked>false</masked>
      <name>state_tag</name>
   </variables>
   <variables>
      <defaultValue>findTestData('NG Game Release Acceptance Test').getValue(4, 1)</defaultValue>
      <description></description>
      <id>ac6ff74a-95b6-4de5-890e-aea8f5360944</id>
      <masked>false</masked>
      <name>game_code</name>
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

def spin = new groovy.json.JsonSlurper()
def result_spin = spin.parseText(response.getResponseBodyContent())

def rgssessiontoken = result_spin.session_token
println (&quot;...value extracted is :&quot;+rgssessiontoken)
GlobalVariable.rgs_session_token = rgssessiontoken
println (&quot;session is :&quot;+GlobalVariable.rgs_session_token)

def statetag = result_spin.state_tag
println (&quot;...value extracted is :&quot;+statetag)
GlobalVariable.state_tag = statetag
println (&quot;state tag is :&quot;+GlobalVariable.state_tag)

def playerid = result_spin.player_id
println (&quot;...value extracted is :&quot;+playerid)
GlobalVariable.player_id = playerid
println (&quot;player id is :&quot;+GlobalVariable.player_id)

def features = result_spin.features
println (&quot;...value extracted is :&quot;+features)
GlobalVariable.features = features
println (&quot;features is :&quot;+GlobalVariable.features)

if (features != null) {
	def free_spin_pick = result_spin.features[0].complete
	println (&quot;free spin pick is :&quot;+free_spin_pick)
	def free_spin_complete = result_spin.features[1].complete
	println (&quot;free spin complete is :&quot;+free_spin_complete)
	def free_spin_left = result_spin.features[1].feature_state.free_spins_left
	println (&quot;free spins left is :&quot;+free_spin_left)
}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
