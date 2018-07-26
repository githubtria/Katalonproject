import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser('')

WebUI.navigateToUrl('https://opensource-demo.orangehrmlive.com/')

WebUI.setText(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/input_txtUsername (1)'), 'Admin')

WebUI.setEncryptedText(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/input_txtPassword (1)'), 'hUKwJTbofgPU9eVlw/CnDQ==')

WebUI.click(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/input_Submit (1)'))

WebUI.click(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/b_Time (1)'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/select_-- Select --MondayTuesd'), 
    '1', true)

WebUI.click(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/b_Recruitment (1)'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/select_AllApplication Initiate'), 
    'SHORTLISTED', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/select_AllSteven Edwards'), '4', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/select_Allceo'), '1', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/select_AllAccount ClerkCEOFina'), 
    '4', true)

WebUI.click(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/li_Method of Application  AllM'))

WebUI.click(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/ol_Job Title  AllAccount Clerk'))

WebUI.click(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/img_ui-datepicker-trigger'))

WebUI.click(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/td_11'))

WebUI.click(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/b_Performance'))

WebUI.click(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/b_Performance'))

WebUI.click(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/b_Performance'))

WebUI.click(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/b_Dashboard (1)'))

WebUI.click(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/a_Welcome Admin'))

WebUI.click(findTestObject('Object Repository/Orange HRM Test/Page_OrangeHRM/li_Logout'))

WebUI.closeBrowser()

