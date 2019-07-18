<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>RGS Detail History - Original</name>
   <tag></tag>
   <elementGuidId>343c464c-12ff-461b-a27c-9701cffc67a5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://c304afdf-2f61-6369-c088-924f99e1be1a-rgs-history.star88ad.com/api/slot/roundid/5d1f088832fbc3000145a157-2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


def rgs_detail_history = new groovy.json.JsonSlurper()
def result_rgs_detail_history = rgs_detail_history.parseText(response.getResponseBodyContent())

def round_id = result_rgs_detail_history.round_id
GlobalVariable.rgs_round_id = round_id
def transaction_id = result_rgs_detail_history.spin_results[0].transaction_id
GlobalVariable.rgs_transaction_id = transaction_id
def transaction_time = result_rgs_detail_history.spin_results[0].transaction_time
GlobalVariable.rgs_transaction_time = transaction_time
def total_bet = result_rgs_detail_history.spin_results[0].total_bet
GlobalVariable.rgs_total_bet = total_bet
def total_value = result_rgs_detail_history.spin_results[0].total_value
GlobalVariable.rgs_bet_value = total_value
def total_win = result_rgs_detail_history.spin_results[0].total_win
GlobalVariable.rgs_total_win = total_win
def balance = result_rgs_detail_history.spin_results[0].balance
GlobalVariable.rgs_balance = balance
def line = result_rgs_detail_history.spin_results[0].line
GlobalVariable.rgs_line = line
def reels = result_rgs_detail_history.spin_results[0].reels
GlobalVariable.rgs_reels = reels
def reel_wins = result_rgs_detail_history.spin_results[0].reel_wins
GlobalVariable.rgs_reel_wins = reel_wins
def scatter_win = result_rgs_detail_history.spin_results[0].scatter_win
GlobalVariable.rgs_scatter_win = scatter_win
def transaction_type = result_rgs_detail_history.spin_results[0].transaction_type
GlobalVariable.rgs_transaction_type = transaction_type
def features_triggered = result_rgs_detail_history.spin_results[0].features_triggered
GlobalVariable.rgs_features_triggered = features_triggered
def player_id = result_rgs_detail_history.player_id
GlobalVariable.rgs_player_id = player_id
def currency = result_rgs_detail_history.currency
GlobalVariable.rgs_currency = currency
def booster_type = result_rgs_detail_history.booster_type
GlobalVariable.rgs_booster_type = booster_type

String detail_tx_time = GlobalVariable.rgs_transaction_time
println(detail_tx_time)
String [] tx_time = detail_tx_time.split(&quot;\\s+&quot;)
println(tx_time)
String rgs_tx_time = tx_time[0]
println(rgs_tx_time)
GlobalVariable.rgs_transaction_time = rgs_tx_time</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
