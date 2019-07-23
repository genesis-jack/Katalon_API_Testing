<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>M4_Spin-Original</name>
   <tag></tag>
   <elementGuidId>2af73410-ce97-40e8-91c8-40312cf9cc65</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;channel\&quot;: \&quot;desktop\&quot;,\n    \&quot;bet\&quot;: 48,\n    \&quot;lines\&quot;: 25,\n    \&quot;multiplier\&quot;: 1,\n    \&quot;currency\&quot;: \&quot;CNY\&quot;,\n    \&quot;points\&quot;: null,\n    \&quot;key\&quot;: null\n}&quot;,
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
   <restUrl>https://${partner}.${env}.com/m4/gameservice/play/spin/77fe001cc7b570b40f337a02ca9ec360cebf5eb8</restUrl>
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
      <defaultValue>'eyJrcnVnX3Nlc3Npb24iOiIyMDFkNDYwYy1hNmEwLTQ5YjctOWNlOS1iMGY1MjI3YjVjY2YiLCJ1c2VyX2lkIjoiODY1ODY2ODAiLCJwYXJ0bmVyX2NvZGUiOiJCQklOIiwiZ2FtZV9rZXkiOiJNNC0wMDEyIiwiY2xpZW50X2lwIjpudWxsLCJtZXJjaGFudF9jb2RlIjoiIiwiZGV2aWNlIjpudWxsLCJjdXJyZW5jeSI6IkNOWSIsImRlbW9fdWlkIjpudWxsLCJwbGF5ZXJfdHlwZSI6IlJFR1VMQVIiLCJiZXRfZGVub21fZGVmYXVsdCI6MH0='</defaultValue>
      <description></description>
      <id>3d78af46-8c57-4f0a-8cbe-30e84e7bbd70</id>
      <masked>false</masked>
      <name>user_id</name>
   </variables>
   <variables>
      <defaultValue>'3655oule'</defaultValue>
      <description></description>
      <id>a5e850eb-f044-47cd-8025-813a7ba9e183</id>
      <masked>false</masked>
      <name>env</name>
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

def reels = result_spin.spin_result.reels
def reel3_sym1 = result_spin.spin_result.reels[2].symbols[0].reel_position
def reel3_sym2 = result_spin.spin_result.reels[2].symbols[1].reel_position
def reel3_sym3 = result_spin.spin_result.reels[2].symbols[2].reel_position
def reel4_sym1 = result_spin.spin_result.reels[3].symbols[0].reel_position
def reel4_sym2 = result_spin.spin_result.reels[3].symbols[1].reel_position
def reel4_sym3 = result_spin.spin_result.reels[3].symbols[2].reel_position
def reel5_sym1 = result_spin.spin_result.reels[4].symbols[0].reel_position
def reel5_sym2 = result_spin.spin_result.reels[4].symbols[1].reel_position
def reel5_sym3 = result_spin.spin_result.reels[4].symbols[2].reel_position

def reel3 = [reel3_sym1, reel3_sym2, reel3_sym3].toArray()
println(reel3)
if ((reel3.contains(4))
	|| (reel3.contains(7))
	|| (reel3.contains(10))
	|| (reel3.contains(13))
	|| (reel3.contains(16))
	|| (reel3.contains(26))) {
	def reel3_has_wild = 'True'
	GlobalVariable.reel3_w = reel3_has_wild
	println(GlobalVariable.reel3_w)
	}
else {
	def reel3_has_wild = 'False'
	GlobalVariable.reel3_w = reel3_has_wild
	println(GlobalVariable.reel3_w)
	}


def reel4 = [reel4_sym1, reel4_sym2, reel4_sym3].toArray()
println(reel4)
if ((reel4.contains(4))
	|| (reel4.contains(7))
	|| (reel4.contains(10))
	|| (reel4.contains(13))
	|| (reel4.contains(16))) {
	def reel4_has_wild = 'True'
	GlobalVariable.reel4_w = reel4_has_wild
	println(GlobalVariable.reel4_w)
	}
else {
	def reel4_has_wild = 'False'
	GlobalVariable.reel4_w = reel4_has_wild
	println(GlobalVariable.reel4_w)
	}

def reel5 = [reel5_sym1, reel5_sym2, reel5_sym3].toArray()
println(reel5)
if ((reel5.contains(4))
	|| (reel5.contains(7))
	|| (reel5.contains(10))
	|| (reel5.contains(13))) {
	def reel5_has_wild = 'True'
	GlobalVariable.reel5_w = reel5_has_wild
	println(GlobalVariable.reel5_w)
	}
else {
	def reel5_has_wild = 'False'
	GlobalVariable.reel5_w = reel5_has_wild
	println(GlobalVariable.reel5_w)
	}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
