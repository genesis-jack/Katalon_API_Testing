<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>M4_Spin</name>
   <tag></tag>
   <elementGuidId>6ab7b8ac-d5d5-4a93-860f-075ab44b04d3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;channel\&quot;: \&quot;desktop\&quot;,\n    \&quot;bet\&quot;: ${betvalue},\n    \&quot;lines\&quot;: 25,\n    \&quot;multiplier\&quot;: 1,\n    \&quot;currency\&quot;: \&quot;CNY\&quot;,\n    \&quot;points\&quot;: null,\n    \&quot;key\&quot;: null\n}&quot;,
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
      <name>X-Genesis-UserId</name>
      <type>Main</type>
      <value>${user_id}</value>
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
   <restUrl>https://${partner}.${env}.com/m4/gameservice/play/spin/${rgs_session_token}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'c304afdf-2f61-6369-c088-924f99e1be1a'</defaultValue>
      <description></description>
      <id>c21027af-d891-4032-871a-850ea4b122cc</id>
      <masked>false</masked>
      <name>partner</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.m4_user_id</defaultValue>
      <description></description>
      <id>59aa48af-7f94-40fc-983c-5ea38bb0dbb7</id>
      <masked>false</masked>
      <name>user_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.rgs_session_token</defaultValue>
      <description></description>
      <id>77cf6511-88ad-4de2-ad16-a58032f5f03d</id>
      <masked>false</masked>
      <name>rgs_session_token</name>
   </variables>
   <variables>
      <defaultValue>'3655oule'</defaultValue>
      <description></description>
      <id>6d58dc42-aac9-4b2c-8960-3888c94f6e46</id>
      <masked>false</masked>
      <name>env</name>
   </variables>
   <variables>
      <defaultValue>'800'</defaultValue>
      <description></description>
      <id>66e11869-db59-40df-a14f-cf016d1779f8</id>
      <masked>false</masked>
      <name>betvalue</name>
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
String newline = System.getProperty(&quot;line.separator&quot;)

def current_balance = result_spin.current_balance
//println(&quot;**** Current Balance ****&quot; + newline + current_balance)
GlobalVariable.current_balance = current_balance
//println(&quot;**** Current Balance ****&quot; + newline + GlobalVariable.current_balance)

def currency = result_spin.currency
//println(&quot;**** Currency ****&quot; + newline + currency)
GlobalVariable.currency = currency
//println(&quot;**** Currency ****&quot; + newline + GlobalVariable.currency)

def round_id = result_spin.spin_result.round_id
//println(&quot;**** Round ID ****&quot; + newline + round_id)
GlobalVariable.round_id = round_id
//println(&quot;**** Round ID ****&quot; + newline + GlobalVariable.round_id)

def total_win = result_spin.spin_result.total_win
//println(&quot;**** Total Win ****&quot; + newline + total_win)
GlobalVariable.rgs_total_win = total_win
//println(&quot;**** Total Win ****&quot; + newline + GlobalVariable.rgs_total_win)


// v.2 defined by symbo name with counter
def wild = new String[5]
wild[0] = &quot;WILD_Expanding&quot;
wild[1] = &quot;WILD_Normal&quot;
wild[2] = &quot;WILD_Double&quot;
wild[3] = &quot;WILD_ExtraFG&quot;
wild[4] = &quot;WILD_Triple&quot;
// java index
def wildList = Arrays.asList(wild)

def counter = new int[3]
counter[0] = 0 //reel 3 wild found
counter[1] = 0
counter[2] = 0

def reels = result_spin.spin_result.reels
for (int i=0; i &lt; reels.size(); i++) {
	def symbols = reels[i].symbols
	for (int j=0; j &lt; symbols.size(); j++) {
		def symbol = symbols[j].symbol
		println(symbol)
		if(i == 2 &amp;&amp; wildList.indexOf(symbol) != -1) {
			println(&quot;wild found on reel 3, adding counter by 1&quot;)
			counter[0] = counter[0] + 1
		}
		if(i == 3 &amp;&amp; wildList.indexOf(symbol) != -1) {
			println(&quot;wild found on reel 4, adding counter by 1&quot;)
			counter[1] = counter[1] + 1
		}
		if(i == 4 &amp;&amp; wildList.indexOf(symbol) != -1) {
			println(&quot;wild found on reel 5, adding counter by 1&quot;)
			counter[2] = counter[2] + 1
		}
	}
}

GlobalVariable.symbo_counter = counter
println(&quot;reel 3 wild found: &quot; + counter[0])
println(&quot;reel 4 wild found: &quot; + counter[1])
println(&quot;reel 5 wild found: &quot; + counter[2])

// v.1
//def reels = result_spin.spin_result.reels
//def reel3_sym1 = result_spin.spin_result.reels[2].symbols[0].reel_position
//def reel3_sym2 = result_spin.spin_result.reels[2].symbols[1].reel_position
//def reel3_sym3 = result_spin.spin_result.reels[2].symbols[2].reel_position
//def reel4_sym1 = result_spin.spin_result.reels[3].symbols[0].reel_position
//def reel4_sym2 = result_spin.spin_result.reels[3].symbols[1].reel_position
//def reel4_sym3 = result_spin.spin_result.reels[3].symbols[2].reel_position
//def reel5_sym1 = result_spin.spin_result.reels[4].symbols[0].reel_position
//def reel5_sym2 = result_spin.spin_result.reels[4].symbols[1].reel_position
//def reel5_sym3 = result_spin.spin_result.reels[4].symbols[2].reel_position
//def reel3 = [reel3_sym1, reel3_sym2, reel3_sym3].toArray()
//println(reel3)
//if ((reel3.contains(4))
//	|| (reel3.contains(7))
//	|| (reel3.contains(10))
//	|| (reel3.contains(13))
//	|| (reel3.contains(16))
//	|| (reel3.contains(26))) {
//	def reel3_has_wild = 'True'
//	GlobalVariable.reel3_w = reel3_has_wild
//	}
//else {
//	def reel3_has_wild = 'False'
//	GlobalVariable.reel3_w = reel3_has_wild
//	}
//
//
//def reel4 = [reel4_sym1, reel4_sym2, reel4_sym3].toArray()
//println(reel4)
//if ((reel4.contains(4))
//	|| (reel4.contains(7))
//	|| (reel4.contains(10))
//	|| (reel4.contains(13))
//	|| (reel4.contains(16))) {
//	def reel4_has_wild = 'True'
//	GlobalVariable.reel4_w = reel4_has_wild
//	}
//else {
//	def reel4_has_wild = 'False'
//	GlobalVariable.reel3_w = reel4_has_wild
//	}
//
//def reel5 = [reel5_sym1, reel5_sym2, reel5_sym3].toArray()
//println(reel5)
//if ((reel5.contains(4))
//	|| (reel5.contains(7))
//	|| (reel5.contains(10))
//	|| (reel5.contains(13))) {
//	def reel5_has_wild = 'True'
//	GlobalVariable.reel5_w = reel5_has_wild
//	}
//else {
//	def reel5_has_wild = 'False'
//	GlobalVariable.reel5_w = reel5_has_wild
//	}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
