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
import net.bytebuddy.implementation.bytecode.constant.NullConstant

response = WS.sendRequestAndVerify(findTestObject('Get_Session_Token'))

WS.sendRequestAndVerify(findTestObject('Login'))

def features = GlobalVariable.features
println('features = ' + features)

def free_spin_pick = GlobalVariable.free_spin_pick
println('fress spin pick is = ' + free_spin_pick)

def free_spin_complete = GlobalVariable.free_spin_complete
println('free spin complete is ' + free_spin_complete)

def free_spin_left = GlobalVariable.free_spin_left
println('free spin left = ' + free_spin_left)

if (features != null) {			// 若觸發 Free Spin
    println('features = ' + features)
	if (free_spin_pick != true) {
	println('free spin pick is = ' + free_spin_pick)
		//WS.sendRequestAndVerify(findTestObject('take-turn_Pick'))
    } 
	else if (free_spin_pick == true) {
	println('free spin pick is = ' + free_spin_pick)
		
	}
//	else if (free_spin_complete == false) {
//        println('free spin complete = ' + free_spin_complete)
//
//        for (int f = 1; f <= free_spin_left; f++) {
//            WS.sendRequestAndVerify(findTestObject('take-turn_FreeSpin'))
//        }
//    }
}

else if (features == null){		// 若沒有觸發 Free Spin
}

