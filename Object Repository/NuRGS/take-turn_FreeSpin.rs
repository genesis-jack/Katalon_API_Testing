<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>take-turn_FreeSpin</name>
   <tag></tag>
   <elementGuidId>25d4623a-d825-470c-91dd-e7e378171a52</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;player_id\&quot;: \&quot;${player_id}\&quot;,\n    \&quot;session_token\&quot;: \&quot;${rgs_session_token}\&quot;,\n    \&quot;partner_code\&quot;: \&quot;${partner_code}\&quot;,\n    \&quot;game_code\&quot;: \&quot;${game_code}\&quot;,\n    \&quot;action\&quot;: \&quot;FREE_SPIN\&quot;,\n    \&quot;state_tag\&quot;: \&quot;${state_tag}\&quot;\n}&quot;,
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
   <restUrl>https://nurgs.${env}.com/ng/take-turn/</restUrl>
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
      <id>95e6890f-4cd3-4ca0-ab30-6d5506d2c5d1</id>
      <masked>false</masked>
      <name>game_code</name>
   </variables>
   <variables>
      <defaultValue>'3655oule'</defaultValue>
      <description></description>
      <id>ba1d338d-eedd-4f4e-b63d-a1fc1bf1b6cf</id>
      <masked>false</masked>
      <name>env</name>
   </variables>
   <verificationScript>// FREE SPIN
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
// println (&quot;**** RGS Session Token is **** :&quot;+newline+&quot;**** &quot;+rgssessiontoken+&quot; ****&quot;)
GlobalVariable.rgs_session_token = rgssessiontoken
// println (&quot;**** RGS Session Token is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.rgs_session_token+&quot; ****&quot;)

def statetag = result_spin.state_tag
// println (&quot;**** State Tag is **** :&quot;+newline+&quot;**** &quot;+statetag+&quot; ****&quot;)
GlobalVariable.state_tag = statetag
// println (&quot;**** State Tag is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.state_tag+&quot; ****&quot;)

def playerid = result_spin.player_id
// println (&quot;**** Player ID is **** :&quot;+newline+&quot;**** &quot;+playerid+&quot; ****&quot;)
GlobalVariable.player_id = playerid
// println (&quot;**** Player ID is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.player_id+&quot; ****&quot;)

def features = result_spin.features
// println (&quot;**** Features is **** :&quot;+newline+&quot;**** &quot;+features+&quot; ****&quot;)
GlobalVariable.features = features
// println (&quot;**** Features is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.features+&quot; ****&quot;)

def balance = result_spin.balance
// println (&quot;**** Balance is **** :&quot;+newline+&quot;**** &quot;+balance+&quot; ****&quot;)
GlobalVariable.balance = balance
// println (&quot;**** Balance is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.balance+&quot; ****&quot;)

def round_id = result_spin.round_id
GlobalVariable.round_id = round_id
def currency = result_spin.currency
GlobalVariable.currency = currency
def fraction = result_spin.fraction
GlobalVariable.fraction = fraction
def partner_code = result_spin.partner_code
GlobalVariable.partner_code = partner_code
def player_id = result_spin.player_id
GlobalVariable.player_id = player_id
def game_code = result_spin.game_code
GlobalVariable.game_code = game_code
def provider = result_spin.provider
GlobalVariable.provider = provider
def round_win = result_spin.round_win
GlobalVariable.round_win = round_win
def feature_win = result_spin.feature_win
GlobalVariable.feature_win = feature_win
def scatter_win = result_spin.scatter_win
GlobalVariable.scatter_win = scatter_win
def bet_amount = result_spin.bet_amount
GlobalVariable.bet_amount = bet_amount
def bet_value = result_spin.bet_value
GlobalVariable.bet_value = bet_value
def lines = result_spin.lines
GlobalVariable.lines = lines
def bet_per_line = result_spin.bet_per_line
GlobalVariable.bet_per_line = bet_per_line
def causality = result_spin.spin_result.causality
GlobalVariable.causality = causality
def reels = result_spin.spin_result.reels
GlobalVariable.reels = reels
def reel_wins = result_spin.spin_result.reel_wins
GlobalVariable.reel_wins = reel_wins
def spin_result_scatter_win = result_spin.spin_result.scatter_win
GlobalVariable.spin_result_scatter_win = spin_result_scatter_win
def win_amount = result_spin.spin_result.win_amount
GlobalVariable.win_amount = win_amount
def engagements = result_spin.engagements
GlobalVariable.engagements = engagements
def features_triggered = result_spin.spin_result.features_triggered
GlobalVariable.features_triggered = features_triggered

if (features != null) {
	// Features Triggered
	def features_type = result_spin.features[0].type
	GlobalVariable.features_type = features_type
	// println (&quot;**** Features is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.features_type+&quot; ****&quot;)

	if ('PICK'.equals(features_type)) {
		// Features Is PICK
		def free_spin_pick = result_spin.features[0].complete
		GlobalVariable.free_spin_pick = free_spin_pick
		// println (&quot;**** Free Spin Pick is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.free_spin_pick+&quot; ****&quot;)

		if (free_spin_pick == true) {
			// Free Spin Picked
			def free_spin_complete = result_spin.features[1].complete
			GlobalVariable.free_spin_complete = free_spin_complete
			// println (&quot;**** Free Spin Complete is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.free_spin_complete+&quot; ****&quot;)
			def free_spin_left = result_spin.features[1].feature_state.free_spins_left
			GlobalVariable.free_spin_left = free_spin_left
			// println (&quot;**** Free Spin Left is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.free_spin_left+&quot; ****&quot;)
		}
	}
	else if ('FREE_SPIN'.equals(features_type)) {
		// Features Is FREE_SPIN
		def free_spin_complete = result_spin.features[0].complete
		GlobalVariable.free_spin_complete = free_spin_complete
		// println (&quot;**** Free Spin Complete is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.free_spin_complete+&quot; ****&quot;)
		def free_spin_left = result_spin.features[0].feature_state.free_spins_left
		GlobalVariable.free_spin_left = free_spin_left
		// println (&quot;**** Free Spin Left is **** :&quot;+newline+&quot;**** &quot;+GlobalVariable.free_spin_left+&quot; ****&quot;)
	}
}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
