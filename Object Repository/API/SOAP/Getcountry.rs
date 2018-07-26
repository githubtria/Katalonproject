<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Getcountry</name>
   <tag></tag>
   <elementGuidId>a727fb79-4900-41b3-b3d7-4f8c9975ac80</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>This XML file does not appear to have any style information associated with it. The document tree is shown below.
&lt;resource xmlns:xlink=&quot;http://www.w3.org/1999/xlink&quot;>
&lt;script/>
&lt;script/>
&lt;CUSTOMERList xlink:href=&quot;http://www.thomas-bayer.com/sqlrest/CUSTOMER/&quot;>CUSTOMER&lt;/CUSTOMERList>
&lt;INVOICEList xlink:href=&quot;http://www.thomas-bayer.com/sqlrest/INVOICE/&quot;>INVOICE&lt;/INVOICEList>
&lt;ITEMList xlink:href=&quot;http://www.thomas-bayer.com/sqlrest/ITEM/&quot;>ITEM&lt;/ITEMList>
&lt;PRODUCTList xlink:href=&quot;http://www.thomas-bayer.com/sqlrest/PRODUCT/&quot;>PRODUCT&lt;/PRODUCTList>
&lt;/resource></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>ListOfContinentsByName</soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable
</verificationScript>
   <wsdlAddress>http://www.thomas-bayer.com/sqlrest/</wsdlAddress>
</WebServiceRequestEntity>
