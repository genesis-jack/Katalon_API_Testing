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

WS.sendRequestAndVerify(findTestObject('Wallet/Get_Session_Token', [('env') : Env_krug_gw
			, ('partner') : Partner, ('secretkey') : Secret_Key, ('player_id') : Player_ID]))
WS.sendRequestAndVerify(findTestObject('NuRGS/Login', [('env') : Env_RGS, ('session_token') : GlobalVariable.session_token
			, ('partner') : Partner, ('game_code') : Game_Code]))

def features = GlobalVariable.features
def features_type = GlobalVariable.features_type
def free_spin_pick = GlobalVariable.free_spin_pick
def free_spin_complete = GlobalVariable.free_spin_complete
def free_spin_left = GlobalVariable.free_spin_left

assert features_type == 'FREE_SPIN' || features_type == 'PICK'