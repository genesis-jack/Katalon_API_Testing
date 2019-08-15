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
import net.bytebuddy.implementation.bytecode.constant.NullConstant as NullConstant

WS.sendRequestAndVerify(findTestObject('Wallet/Get_Session_Token', [('env') : Env_krug_gw, ('partner') : Partner, ('secretkey') : Secret_Key, ('player_id') : Player_ID]))

WS.sendRequestAndVerify(findTestObject('NuRGS/Login', [('env') : Env_RGS, ('session_token') : GlobalVariable.session_token, ('partner') : Partner, ('game_code') : Game_Code]))

for (int i = 0; i <= 1; i++) {
	def features = GlobalVariable.features
	def features_type = GlobalVariable.features_type
	def free_spin_pick = GlobalVariable.free_spin_pick
	def free_spin_complete = GlobalVariable.free_spin_complete
	def free_spin_left = GlobalVariable.free_spin_left
	String newline = System.getProperty("line.separator")
	
	if (features == null) {
		// Feature Not Triggered
		println('**** In Test Case - Feature is ****'+newline+features)
		println('**** In Test Case - Complete = **** '+newline+free_spin_complete)
		spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_BaseSpin', [('env') : Env_RGS, ('player_id') : GlobalVariable.player_id
					, ('partner_code') : GlobalVariable.partner_code, ('rgs_session_token') : GlobalVariable.rgs_session_token
					, ('state_tag') : GlobalVariable.state_tag, ('game_code') : Game_Code]))
	}

	else if ((features != null) && ('FREE_SPIN'.equals(features_type) && (free_spin_complete != true))) {
		// Feature Triggered And Type Is "FREE_SPIN" And Not Complete
		println('**** In Test Case - Feature is ****'+newline+features)
		println('**** In Test Case - Complete = **** '+newline+free_spin_complete)
		spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_FreeSpin', [('env') : Env_RGS, ('player_id') : GlobalVariable.player_id
					, ('partner_code') : GlobalVariable.partner_code, ('rgs_session_token') : GlobalVariable.rgs_session_token
					, ('state_tag') : GlobalVariable.state_tag, ('game_code') : Game_Code]))
		println('**** In Test Case - Free Spin Left **** = '+newline+GlobalVariable.free_spin_left)
	}

	else if ((features != null) && ('FREE_SPIN'.equals(features_type) && (free_spin_complete == true))) {
		// Feature Triggered And Type Is "FREE_SPIN" And Complete
		println('**** In Test Case - Feature is ****'+newline+features)
		spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_BaseSpin', [('env') : Env_RGS, ('player_id') : GlobalVariable.player_id
					, ('partner_code') : GlobalVariable.partner_code, ('rgs_session_token') : GlobalVariable.rgs_session_token
					, ('state_tag') : GlobalVariable.state_tag, ('game_code') : Game_Code]))
		println('**** In Test Case - Free Spin Left **** = '+newline+GlobalVariable.free_spin_left)
	}
	
	else if ((features != null) && 'PICK'.equals(features_type)) {
		// Feature Triggered And Type Is "PICK"
		println('**** In Test Case - Feature is ****'+newline+features)
		println('**** In Test Case - Free Spin Pick is ****'+newline+free_spin_pick)
		
		if (free_spin_pick != true) {
			// Not Pick Yet
			spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_Pick', [('env') : Env_RGS, ('player_id') : GlobalVariable.player_id
						, ('partner_code') : GlobalVariable.partner_code, ('rgs_session_token') : GlobalVariable.rgs_session_token
						, ('state_tag') : GlobalVariable.state_tag, ('game_code') : Game_Code]))
		}
		
		else if (free_spin_pick == true) {
			// Picked
		println('**** In Test Case - Feature is ****'+newline+features)
		println('**** In Test Case - Free Spin Pick is ****'+newline+free_spin_pick)

			if (free_spin_complete != true) {
				// Free Spin Not Complete
				spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_FreeSpin', [('env') : Env_RGS, ('player_id') : GlobalVariable.player_id
							, ('partner_code') : GlobalVariable.partner_code, ('rgs_session_token') : GlobalVariable.rgs_session_token
							, ('state_tag') : GlobalVariable.state_tag, ('game_code') : Game_Code]))
				println('**** In Test Case - Free Spin Left is ****'+newline+GlobalVariable.free_spin_left)
			}
			
			else if (free_spin_complete == true) {
				// Free Spin Complete
				println('**** In Test Case - Feature is ****'+newline+features)
				spin_result = WS.sendRequestAndVerify(findTestObject('NuRGS/take-turn_BaseSpin', [('env') : Env_RGS, ('player_id') : GlobalVariable.player_id
							, ('partner_code') : GlobalVariable.partner_code, ('rgs_session_token') : GlobalVariable.rgs_session_token
							, ('state_tag') : GlobalVariable.state_tag, ('game_code') : Game_Code]))
			}
		}
	}
}