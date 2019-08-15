<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Player Transaction History-Free Spin</name>
   <tag></tag>
   <elementGuidId>d22811bf-30f0-4c7d-9363-4c7f68ca9042</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://${env}/history/transactions/all/players/${player_id}?partnerToken=${partner}&amp;startDate=${tx_time}T00:00:00Z&amp;endDate=${tx_time}T23:59:59.999Z&amp;pageNumber=1&amp;pageSize=10&amp;action=ROLLBACK%2CAdjustment_Add%2CAdjustment_Remove%2CDeposit%2CWithdraw</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.partner</defaultValue>
      <description></description>
      <id>08e7c0ba-8fae-467b-b564-c46c62aed374</id>
      <masked>false</masked>
      <name>partner</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.player_id</defaultValue>
      <description></description>
      <id>982b9ddf-cb9d-4b96-8461-0950531df2e3</id>
      <masked>false</masked>
      <name>player_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.rgs_transaction_time</defaultValue>
      <description></description>
      <id>8ba8e3cc-6054-43cd-a66e-d3cc1f518529</id>
      <masked>false</masked>
      <name>tx_time</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Environment').getValue(5, 1)</defaultValue>
      <description></description>
      <id>634305ff-10c1-4187-b663-9ae3b74c02a2</id>
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

def player_transaction_history = new groovy.json.JsonSlurper()
def result_player_transaction_history = player_transaction_history.parseText(response.getResponseBodyContent())
println(result_player_transaction_history)
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

def betPaid = result_player_transaction_history.documents.payload[0].betPaid
double betPaid_pth = Double.parseDouble(betPaid)
GlobalVariable.pth_betPaid = betPaid_pth

def betValue = result_player_transaction_history.documents.payload[0].betValue
double betValue_pth = Double.parseDouble(betValue)
GlobalVariable.pth_betValue = betValue_pth

def win = result_player_transaction_history.documents.payload[0].win
double win_pth = Double.parseDouble(win)
GlobalVariable.pth_win = win_pth

def winLoss = result_player_transaction_history.documents.payload[0].winLoss
double winLoss_pth = Double.parseDouble(winLoss)
GlobalVariable.pth_winLoss = winLoss_pth

def totalWinLoss = result_player_transaction_history.documents.summary.totalWinLoss
double totalWinLoss_pth = Double.parseDouble(totalWinLoss)
GlobalVariable.pth_totalWinLoss = totalWinLoss_pth

def balance = result_player_transaction_history.documents.payload[0].balance
double balance_pth = Double.parseDouble(balance)
GlobalVariable.pth_balance = balance_pth

def totalBetPaid = result_player_transaction_history.documents.summary.totalBetPaid
double totalBetPaid_pth = Double.parseDouble(totalBetPaid)
GlobalVariable.pth_totalBetPaid = totalBetPaid_pth

def totalBetValue = result_player_transaction_history.documents.summary.totalBetValue
double totalBetValue_pth = Double.parseDouble(totalBetValue)
GlobalVariable.pth_totalBetValue = totalBetValue_pth

def totalWin = result_player_transaction_history.documents.summary.totalWin
double totalWin_pth = Double.parseDouble(totalWin)
GlobalVariable.pth_totalWin = totalWin_pth

def currency = result_player_transaction_history.documents.payload[0].currency
GlobalVariable.pth_currency = currency
def timeStamp = result_player_transaction_history.documents.payload[0].timeStamp
GlobalVariable.pth_timeStamp = timeStamp
def totalNum = result_player_transaction_history.documents.summary.totalNum
GlobalVariable.pth_totalNum = totalNum</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
