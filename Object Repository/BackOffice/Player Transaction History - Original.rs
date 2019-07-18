<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Player Transaction History - Original</name>
   <tag></tag>
   <elementGuidId>db093454-0fcf-4257-ab20-49efef471463</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://krug-gw.star9ad.com/history/transactions/all/players/${player_id}?partnerToken=${partner}&amp;startDate=${rgs_transaction_time}T00:00:00Z&amp;endDate=${rgs_transaction_time}T23:59:59.999Z&amp;pageNumber=1&amp;pageSize=10&amp;action=ROLLBACK%2CAdjustment_Add%2CAdjustment_Remove%2CDeposit%2CWithdraw</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'c304afdf-2f61-6369-c088-924f99e1be1a'</defaultValue>
      <description></description>
      <id>08e7c0ba-8fae-467b-b564-c46c62aed374</id>
      <masked>false</masked>
      <name>partner</name>
   </variables>
   <variables>
      <defaultValue>'Katalon_Player_0001'</defaultValue>
      <description></description>
      <id>982b9ddf-cb9d-4b96-8461-0950531df2e3</id>
      <masked>false</masked>
      <name>player_id</name>
   </variables>
   <variables>
      <defaultValue>'2019-07-12'</defaultValue>
      <description></description>
      <id>8087ba0c-271b-46f4-bbfb-abb0c09feeb0</id>
      <masked>false</masked>
      <name>rgs_transaction_time</name>
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

def player_transaction_history = new groovy.json.JsonSlurper()
def result_player_transaction_history = player_transaction_history.parseText(response.getResponseBodyContent())

def txDate = result_player_transaction_history.documents.payload[0].txDate
GlobalVariable.pth_txDate = txDate
def roundId = result_player_transaction_history.documents.payload[0].roundId
GlobalVariable.pth_roundId = roundId
def playerId = result_player_transaction_history.documents.payload[0].playerId
GlobalVariable.pth_playerId = playerId
def txId = result_player_transaction_history.documents.payload[0].txId
GlobalVariable.pth_txId = txId
def providerId = result_player_transaction_history.documents.payload[0].providerId
GlobalVariable.pth_providerId = providerId
def gameId = result_player_transaction_history.documents.payload[0].gameId
GlobalVariable.pth_gameId = gameId
def gameName = result_player_transaction_history.documents.payload[0].gameName
GlobalVariable.pth_gameName = gameName
def txType = result_player_transaction_history.documents.payload[0].txType
GlobalVariable.pth_txType = txType
def subType = result_player_transaction_history.documents.payload[0].subType
GlobalVariable.pth_subType = subType
def device = result_player_transaction_history.documents.payload[0].device
GlobalVariable.pth_device = device
def winLoss = result_player_transaction_history.documents.payload[0].winLoss
GlobalVariable.pth_winLoss = winLoss
def betPaid = result_player_transaction_history.documents.payload[0].betPaid
GlobalVariable.pth_betPaid = betPaid
def betValue = result_player_transaction_history.documents.payload[0].betValue
GlobalVariable.pth_betValue = betValue
def win = result_player_transaction_history.documents.payload[0].win
GlobalVariable.pth_win = win

def balance = result_player_transaction_history.documents.payload[0].balance
float balance_pth = Float.parseFloat(balance)
GlobalVariable.pth_balance = balance_pth
println(GlobalVariable.pth_balance)

def currency = result_player_transaction_history.documents.payload[0].currency
GlobalVariable.pth_currency = currency
def timeStamp = result_player_transaction_history.documents.payload[0].timeStamp
GlobalVariable.pth_timeStamp = timeStamp
def totalBetPaid = result_player_transaction_history.documents.summary.totalBetPaid
GlobalVariable.pth_totalBetPaid = totalBetPaid
def totalWinLoss = result_player_transaction_history.documents.summary.totalWinLoss
GlobalVariable.pth_totalWinLoss = totalWinLoss
def totalNum = result_player_transaction_history.documents.summary.totalNum
GlobalVariable.pth_totalNum = totalNum
def totalBetValue = result_player_transaction_history.documents.summary.totalBetValue
GlobalVariable.pth_totalBetValue = totalBetValue
def totalWin = result_player_transaction_history.documents.summary.totalWin
GlobalVariable.pth_totalWin = totalWin</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
