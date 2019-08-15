<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BO Transaction</name>
   <tag></tag>
   <elementGuidId>ad195d60-cbb8-4741-b9b4-4519b4309b91</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
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
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://${env}/m4/bo/transaction/${transaction_id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'c304afdf-2f61-6369-c088-924f99e1be1a'</defaultValue>
      <description></description>
      <id>731c189d-2341-4d40-918f-7e06f8cf5d73</id>
      <masked>false</masked>
      <name>partner</name>
   </variables>
   <variables>
      <defaultValue>'965d3197-0f22-40a4-9aa5-4e0cd9a320b0'</defaultValue>
      <description></description>
      <id>dcdb8079-5926-4b28-984c-537ade8b5b33</id>
      <masked>false</masked>
      <name>transaction_id</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Environment').getValue(5, 2)</defaultValue>
      <description></description>
      <id>a22a06f8-70e1-4a6c-b038-4335a8fa4048</id>
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

def bo_transaction = new groovy.json.JsonSlurper()
def result_bo_transaction = bo_transaction.parseText(response.getResponseBodyContent())
def bonusproviderref = result_bo_transaction.bonusproviderref
GlobalVariable.bo_bonusproviderref = bonusproviderref
def roundType = result_bo_transaction.roundType
GlobalVariable.bo_roundType = roundType
def boosterType = result_bo_transaction.boosterType
GlobalVariable.bo_boosterType = boosterType
def bonusBet = result_bo_transaction.bonusBet
GlobalVariable.bo_bonusBet = bonusBet
def betValue = result_bo_transaction.betValue
GlobalVariable.bo_betValue = betValue
def partner_data = result_bo_transaction.partner_data
GlobalVariable.bo_partner_data = partner_data
def user_id = result_bo_transaction.user_id
GlobalVariable.bo_user_id = user_id
def game_id = result_bo_transaction.game_id
GlobalVariable.bo_game_id = game_id
def causality = result_bo_transaction.causality
GlobalVariable.bo_causality = causality
def currency = result_bo_transaction.currency
GlobalVariable.bo_currency = currency
def total_bet = result_bo_transaction.total_bet
GlobalVariable.bo_total_bet = total_bet
def total_won = result_bo_transaction.total_won
GlobalVariable.bo_total_won = total_won
def balance = result_bo_transaction.balance
GlobalVariable.bo_balance = balance
def timestamp = result_bo_transaction.timestamp
GlobalVariable.bo_timestamp = timestamp
def merchantcode = result_bo_transaction.merchantcode
GlobalVariable.bo_merchantcode = merchantcode
def device = result_bo_transaction.device
GlobalVariable.bo_device = device
def user_type = result_bo_transaction.user_type
GlobalVariable.bo_user_type = user_type
def roundid = result_bo_transaction.roundid
GlobalVariable.bo_round_id = roundid
def jp_id = result_bo_transaction.jp_id
GlobalVariable.bo_jp_id = jp_id
def jpcontrib = result_bo_transaction.jpcontrib
GlobalVariable.bo_jpcontrib = jpcontrib</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
