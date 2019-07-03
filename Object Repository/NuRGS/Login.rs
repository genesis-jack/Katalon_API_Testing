<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Login</name>
   <tag></tag>
   <elementGuidId>f224b7c5-3e07-4849-aa6a-736cf5e8bd51</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;partner_token\&quot;: \&quot;${partner}\&quot;,\n    \&quot;player_token\&quot;: \&quot;${session_token}\&quot;,\n    \&quot;game_code\&quot;: \&quot;${game_code}\&quot;,\n    \&quot;device\&quot;: \&quot;DESKTOP\&quot;\n}&quot;,
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
   <restUrl>https://nurgs.star9ad.com/ng/sessions/</restUrl>
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
      <defaultValue>findTestData('Partner_Data').getValue(2, 1)</defaultValue>
      <description></description>
      <id>469e4f86-9bbc-404c-b32b-4922e3199538</id>
      <masked>false</masked>
      <name>partner</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Game_Data').getValue(4, 1)</defaultValue>
      <description></description>
      <id>90bc28d0-e75c-4c4d-a09c-5112c5f3929d</id>
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

def login = new groovy.json.JsonSlurper()
def result_login = login.parseText(response.getResponseBodyContent())

def rgssessiontoken = result_login.state.session_token
println (&quot;...value extracted is :&quot;+rgssessiontoken)
GlobalVariable.rgs_session_token = rgssessiontoken
println (&quot;RGS Session is :&quot;+GlobalVariable.rgs_session_token)

def features = result_login.state.features
println (&quot;...value extracted is :&quot;+features)
GlobalVariable.features = features
println (&quot;Features is :&quot;+GlobalVariable.features)

def statetag = result_login.state.state_tag
println (&quot;...value extracted is :&quot;+statetag)
GlobalVariable.state_tag = statetag
println (&quot;State Tag is :&quot;+GlobalVariable.state_tag)

def playerid = result_login.state.player_id
println (&quot;...value extracted is :&quot;+playerid)
GlobalVariable.player_id = playerid
println (&quot;Player ID is :&quot;+GlobalVariable.player_id)

def partner_code = result_login.state.partner_code
println (&quot;...value extracted is :&quot;+partner_code)
GlobalVariable.partner_code = partner_code
println (&quot;Partner Code is :&quot;+GlobalVariable.partner_code)

if (features != null) {
	// Feature Triggered
	def features_type = result_login.state.features[0].type
	println (&quot;...value extracted is :&quot;+features_type)
	GlobalVariable.features_type = features_type
	
	if (&quot;PICK&quot;.equals(features_type)) {
		// Feature Type Is Pick
		def free_spin_pick = result_login.state.features[0].complete
		println (&quot;...value extracted is :&quot;+free_spin_pick)
		GlobalVariable.free_spin_pick = free_spin_pick
		println (&quot;Pick complete is :&quot;+GlobalVariable.free_spin_pick)

				if (free_spin_pick == true) {
					// Free Spin Picked
					def free_spin_complete = result_login.state.features[1].complete
					println (&quot;...value extracted is :&quot;+free_spin_complete)
					GlobalVariable.free_spin_complete = free_spin_complete
					println (&quot;Pick complete is :&quot;+GlobalVariable.free_spin_complete)
				
					def free_spin_left = result_login.state.features[1].feature_state.free_spins_left
					println (&quot;...value extracted is :&quot;+free_spin_left)
					GlobalVariable.free_spin_left = free_spin_left
					println (&quot;Free Spin Left is :&quot;+GlobalVariable.free_spin_left)
			}
	}
	else if (&quot;FREE_SPIN&quot;.equals(features_type)) {
		// Feature Type Is FREE_SPIN
		def free_spin_complete = result_login.state.features[0].complete
		println (&quot;...value extracted is :&quot;+free_spin_complete)
		GlobalVariable.free_spin_complete = free_spin_complete
		println (&quot;Pick complete is :&quot;+GlobalVariable.free_spin_complete)
	
		def free_spin_left = result_login.state.features[0].feature_state.free_spins_left
		println (&quot;...value extracted is :&quot;+free_spin_left)
		GlobalVariable.free_spin_left = free_spin_left
		println (&quot;Free Spin Left is :&quot;+GlobalVariable.free_spin_left)
	}
}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
