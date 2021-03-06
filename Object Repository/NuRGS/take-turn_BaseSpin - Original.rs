<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>take-turn_BaseSpin - Original</name>
   <tag></tag>
   <elementGuidId>4bfa9ada-7e17-4571-896f-ef5c460d6a05</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n   \&quot;player_id\&quot;: \&quot;XH_gt_gns_03\&quot;,\n   \&quot;partner_code\&quot;: \&quot;${partner_code}\&quot;,\n   \&quot;game_code\&quot;: \&quot;${game_code}\&quot;,\n   \&quot;action\&quot;: \&quot;SPIN\&quot;,\n   \&quot;session_token\&quot;: \&quot;${rgs_session_token}\&quot;,\n   \&quot;state_tag\&quot;:\&quot;${state_tag}\&quot;,\n   \&quot;bet_denom_index\&quot;: 1\n}&quot;,
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
      <defaultValue>'XH_gt_gns_03'</defaultValue>
      <description></description>
      <id>88345572-0624-4879-9ed5-fba5ed710c60</id>
      <masked>false</masked>
      <name>player_id</name>
   </variables>
   <variables>
      <defaultValue>'GT'</defaultValue>
      <description></description>
      <id>5d4093f1-b3be-4438-9382-dae44691ce15</id>
      <masked>false</masked>
      <name>partner_code</name>
   </variables>
   <variables>
      <defaultValue>'06c0c110-5621-46c8-8a0e-399e38593d34'</defaultValue>
      <description></description>
      <id>edf13d5c-239b-4355-803f-4b21a4176c2e</id>
      <masked>false</masked>
      <name>rgs_session_token</name>
   </variables>
   <variables>
      <defaultValue>'B0E30C35'</defaultValue>
      <description></description>
      <id>81775165-c804-481b-873f-5ed32c2334d0</id>
      <masked>false</masked>
      <name>state_tag</name>
   </variables>
   <variables>
      <defaultValue>'NG-0063'</defaultValue>
      <description></description>
      <id>4028fe26-d61e-4908-8e90-5e3cbd963da0</id>
      <masked>false</masked>
      <name>game_code</name>
   </variables>
   <verificationScript>// BASE SPIN
import static org.assertj.core.api.Assertions.*
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

String newline = System.getProperty(&quot;line.separator&quot;)
def spin = new groovy.json.JsonSlurper()
def result_spin = spin.parseText(response.getResponseBodyContent())
result = response.getResponseBodyContent()
println(result)

def rgssessiontoken = result_spin.session_token
println (&quot;**** RGS Session Token is **** :&quot;+newline+&quot;**** &quot;+rgssessiontoken+&quot; ****&quot;)
GlobalVariable.rgs_session_token = rgssessiontoken
println (&quot;**** RGS Session Token is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.rgs_session_token+&quot; ****&quot;)

def statetag = result_spin.state_tag
println (&quot;**** State Tag is **** :&quot;+newline+&quot;**** &quot;+statetag+&quot; ****&quot;)
GlobalVariable.state_tag = statetag
println (&quot;**** State Tag is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.state_tag+&quot; ****&quot;)

def playerid = result_spin.player_id
println (&quot;**** Player ID is **** :&quot;+newline+&quot;**** &quot;+playerid+&quot; ****&quot;)
GlobalVariable.player_id = playerid
println (&quot;**** Player ID is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.player_id+&quot; ****&quot;)

def features = result_spin.features
println (&quot;**** Features is **** :&quot;+newline+&quot;**** &quot;+features+&quot; ****&quot;)
GlobalVariable.features = features
println (&quot;**** Features is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.features+&quot; ****&quot;)

def balance = result_spin.balance
println (&quot;**** Balance is **** :&quot;+newline+&quot;**** &quot;+balance+&quot; ****&quot;)
GlobalVariable.balance = balance
println (&quot;**** Balance is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.balance+&quot; ****&quot;)


if (features != null) {
	// Features Triggered
	def features_type = result_spin.features[0].type
	println (&quot;**** Features is **** :&quot;+newline+&quot;**** &quot;+features+&quot; ****&quot;)
	GlobalVariable.features_type = features_type
	println (&quot;**** Features is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.features_type+&quot; ****&quot;)

	if ('PICK'.equals(features_type)) {
		// Features Is PICK
		def free_spin_pick = result_spin.features[0].complete
		GlobalVariable.free_spin_pick = free_spin_pick
		println (&quot;**** Free Spin Pick is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.free_spin_pick+&quot; ****&quot;)

		if (free_spin_pick == true) {
			// Free Spin Picked
			def free_spin_complete = result_spin.features[1].complete
			GlobalVariable.free_spin_complete = free_spin_complete
			println (&quot;**** Free Spin Complete is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.free_spin_complete+&quot; ****&quot;)
			def free_spin_left = result_spin.features[1].feature_state.free_spins_left
			GlobalVariable.free_spin_left = free_spin_left
			println (&quot;**** Free Spin Left is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.free_spin_left+&quot; ****&quot;)
		}
	}
	else if ('FREE_SPIN'.equals(features_type)) {
		// Features Is FREE_SPIN
		def free_spin_complete = result_spin.features[0].complete
		GlobalVariable.free_spin_complete = free_spin_complete
		println (&quot;**** Free Spin Complete is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.free_spin_complete+&quot; ****&quot;)
		def free_spin_left = result_spin.features[0].feature_state.free_spins_left
		GlobalVariable.free_spin_left = free_spin_left
		println (&quot;**** Free Spin Left is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.free_spin_left+&quot; ****&quot;)
	}
}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
