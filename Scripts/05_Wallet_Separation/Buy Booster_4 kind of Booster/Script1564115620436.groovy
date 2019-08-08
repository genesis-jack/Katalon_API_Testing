import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WS.sendRequestAndVerify(findTestObject('Wallet/Get_Session_Token', [('env') : Env_krug_gw, ('partner') : Partner
            , ('secretkey') : Secret_Key, ('player_id') : Player_ID]))

WS.sendRequestAndVerify(findTestObject('NuRGS/Login', [('env') : Env_RGS, ('session_token') : GlobalVariable.session_token
            , ('game_code') : 'NG-0063', ('partner') : Partner]))

spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_BaseSpin', [('env') : Env_RGS, ('player_id') : Player_ID
			, ('partner_code') : Partner_Code, ('rgs_session_token') : GlobalVariable.rgs_session_token
			, ('state_tag') : GlobalVariable.state_tag, ('game_code') : Game_Code]))

spin_result = WS.sendRequestAndVerify(findTestObject('RadPlus/Buy Booster_Regular', [('env') : Env_RGS, ('partner') : Partner
			, ('partner_code') : Partner_Code, ('rgs_session_token') : GlobalVariable.rgs_session_token, ('player_id') : Player_ID]))

spin_result = WS.sendRequestAndVerify(findTestObject('RadPlus/Buy Booster_Lite', [('env') : Env_RGS, ('partner') : Partner
			, ('partner_code') : Partner_Code, ('rgs_session_token') : GlobalVariable.rgs_session_token, ('player_id') : Player_ID]))

spin_result = WS.sendRequestAndVerify(findTestObject('RadPlus/Buy Booster_Super', [('env') : Env_RGS, ('partner') : Partner
			, ('partner_code') : Partner_Code, ('rgs_session_token') : GlobalVariable.rgs_session_token, ('player_id') : Player_ID]))

spin_result = WS.sendRequestAndVerify(findTestObject('RadPlus/Buy Booster_Wild', [('env') : Env_RGS, ('partner') : Partner
			, ('partner_code') : Partner_Code, ('rgs_session_token') : GlobalVariable.rgs_session_token, ('player_id') : Player_ID]))