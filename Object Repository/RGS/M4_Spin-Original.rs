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
  &quot;text&quot;: &quot;{\n    \&quot;channel\&quot;: \&quot;desktop\&quot;,\n    \&quot;bet\&quot;: \&quot;${bet_value}\&quot;,\n    \&quot;lines\&quot;: 1,\n    \&quot;multiplier\&quot;: 1,\n    \&quot;currency\&quot;: \&quot;CNY\&quot;,\n    \&quot;points\&quot;: null,\n    \&quot;key\&quot;: null\n}&quot;,
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
   <restUrl>https://${partner}.star88ad.com/m4/gameservice/play/spin/934bf32dc36fc357071f6f4c526e8be1aeabcb54</restUrl>
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
      <defaultValue>'eyJrcnVnX3Nlc3Npb24iOiI1ZjIzMDc5Ny03ZWQwLTQyY2EtOTgyNC00NGQxMTIzMjM2YTMiLCJ1c2VyX2lkIjoiV2FsbGV0UGxheWVyMDUiLCJwYXJ0bmVyX2NvZGUiOiJCQklOIiwiZ2FtZV9rZXkiOiJNNC0wMDA4IiwiY2xpZW50X2lwIjpudWxsLCJtZXJjaGFudF9jb2RlIjoiIiwiZGV2aWNlIjpudWxsLCJjdXJyZW5jeSI6IkNOWSIsImRlbW9fdWlkIjpudWxsLCJwbGF5ZXJfdHlwZSI6IkhPVVNFIiwiYmV0X2Rlbm9tX2RlZmF1bHQiOjB9'</defaultValue>
      <description></description>
      <id>3d78af46-8c57-4f0a-8cbe-30e84e7bbd70</id>
      <masked>false</masked>
      <name>user_id</name>
   </variables>
   <variables>
      <defaultValue>'800'</defaultValue>
      <description></description>
      <id>1d324023-7871-4bb1-9da6-5cf5baf19653</id>
      <masked>false</masked>
      <name>bet_value</name>
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
println(&quot;**** Current Balance ****&quot; + newline + current_balance)
GlobalVariable.current_balance = current_balance
println(&quot;**** Current Balance ****&quot; + newline + GlobalVariable.current_balance)

def currency = result_spin.currency
println(&quot;**** Currency ****&quot; + newline + currency)
GlobalVariable.currency = currency
println(&quot;**** Currency ****&quot; + newline + GlobalVariable.currency)

def round_id = result_spin.spin_result.round_id
println(&quot;**** Round ID ****&quot; + newline + round_id)
GlobalVariable.round_id = round_id
println(&quot;**** Round ID ****&quot; + newline + GlobalVariable.round_id)

def total_win = result_spin.spin_result.total_win
println(&quot;**** Total Win ****&quot; + newline + total_win)
GlobalVariable.total_win = total_win
println(&quot;**** Total Win ****&quot; + newline + GlobalVariable.total_win)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
