<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>take-turn_Pick</name>
   <tag></tag>
   <elementGuidId>70281927-c79b-4530-8c58-af74c2b20023</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n   \&quot;player_id\&quot;: \&quot;${player_id}\&quot;,\n   \&quot;partner_code\&quot;: \&quot;${partner_code}\&quot;,\n   \&quot;choice_id\&quot;: \&quot;FS_20\&quot;,\n   \&quot;game_code\&quot;: \&quot;${game_code}\&quot;,\n   \&quot;action\&quot;: \&quot;PICK\&quot;,\n   \&quot;session_token\&quot;: \&quot;${rgs_session_token}\&quot;,\n   \&quot;state_tag\&quot;:\&quot;${state_tag}\&quot;,\n   \&quot;bet_denom_index\&quot;: 1\n}&quot;,
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
      <defaultValue>findTestData('Game_Data').getValue(4, 1)</defaultValue>
      <description></description>
      <id>55370c75-0501-420b-91f0-803170dba38f</id>
      <masked>false</masked>
      <name>game_code</name>
   </variables>
   <verificationScript>// Pick
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
def rgssessiontoken = result_spin.session_token
println (&quot;**** RGS Session is **** :&quot;+newline+&quot;**** &quot;+rgssessiontoken+&quot; ****&quot;)
GlobalVariable.rgs_session_token = rgssessiontoken
println (&quot;**** RGS Session is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.rgs_session_token+&quot; ****&quot;)

def statetag = result_spin.state_tag
println (&quot;**** State Tag is **** :&quot;+newline+&quot;**** &quot;+statetag+&quot; ****&quot;)
GlobalVariable.state_tag = statetag
println (&quot;**** State Tag is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.state_tag+&quot; ****&quot;)

def playerid = result_spin.player_id
println (&quot;**** Player ID is **** :&quot;+newline+&quot;**** &quot;+playerid+&quot; ****&quot;)
GlobalVariable.player_id = playerid
println (&quot;**** Player ID is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.player_id+&quot; ****&quot;)

def features = result_spin.features
println (&quot;**** Features is **** :&quot;+newline+&quot;**** &quot;+features+&quot; *****&quot;)
GlobalVariable.features = features
println (&quot;**** Features is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.features+&quot; ****&quot;)

if (features != null) {		// Free Spin Triggered
	def free_spin_pick = result_spin.features[0].complete
	GlobalVariable.free_spin_pick = free_spin_pick
	println (&quot;**** Free Spin Pick is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.free_spin_pick+&quot; ****&quot;)
	
	if (free_spin_pick == true) {		// Free Spin Picked
		def free_spin_complete = result_spin.features[1].complete
		GlobalVariable.free_spin_complete = free_spin_complete
		println (&quot;**** Free Spin Complete is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.free_spin_complete+&quot; ****&quot;)
		def free_spin_left = result_spin.features[1].feature_state.free_spins_left
		GlobalVariable.free_spin_left = free_spin_left
		println (&quot;**** Free Spins Left is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.free_spin_left+&quot; ****&quot;)
	}
}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
