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

WS.sendRequestAndVerify(findTestObject('Wallet/Get_Session_Token', [('env') : '3655oule', ('partner') : Partner, ('secretkey') : Secret_Key, ('player_id') : Player_ID]))
WS.sendRequestAndVerify(findTestObject('RGS/M4_Login', [('env') : '3655oule', ('session_token') : GlobalVariable.session_token, ('partner') : Partner, ('game_code') : 'M4-0012']))


for (int i = 0; i <= 100;i++) {
	WS.sendRequestAndVerify(findTestObject('RGS/M4_Init', [('env') : '3655oule', ('partner') : Partner, ('user_id') : GlobalVariable.m4_user_id, ('gameId') : 'SW_M4_V1_RECORDER']))
	WS.sendRequestAndVerify(findTestObject('RGS/M4_Spin', [('env') : '3655oule', ('partner') : Partner, ('user_id') : GlobalVariable.m4_user_id
			, ('rgs_session_token') : GlobalVariable.rgs_session_token, ('betvalue') : '800']))
	
	// Free Spin Triggered
	if ((GlobalVariable.symbo_counter[0] >= 1) && (GlobalVariable.symbo_counter[1] >= 1) && (GlobalVariable.symbo_counter[2] >= 1))
	break;
}