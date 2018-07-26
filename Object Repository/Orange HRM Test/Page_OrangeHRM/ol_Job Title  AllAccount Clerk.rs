<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>ol_Job Title  AllAccount Clerk</name>
   <tag></tag>
   <elementGuidId>7eb81304-6058-40c2-bd48-b08c126ccec7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>ol</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                    Job Title
  
All
Account Clerk
CEO
Finance Manager
HR Executive
HR Manager
IT Executive
IT Manager
Sales Executive
Sales Manager


Vacancy
  All

Hiring Manager
  All

Status
  
All
Application Initiated
Shortlisted
Interview Scheduled
Interview Passed
Interview Failed
Job Offered
Offer Declined
Rejected
Hired


Candidate Name
  

Keywords
  

Date of Application
  From  

    var datepickerDateFormat = 'yy-mm-dd';
    var displayDateFormat = datepickerDateFormat.replace('yy', 'yyyy');

    $(document).ready(function(){
        
        var dateFieldValue = $.trim($(&quot;#candidateSearch_fromDate&quot;).val());
        if (dateFieldValue == '') {
            $(&quot;#candidateSearch_fromDate&quot;).val(displayDateFormat);
        }

        daymarker.bindElement(&quot;#candidateSearch_fromDate&quot;,
        {
            showOn: &quot;both&quot;,
            dateFormat: datepickerDateFormat,
            buttonImage: &quot;/webres_5acde3dbd3adc6.90334155/themes/default/images/calendar.png&quot;,
            buttonText:&quot;&quot;,
            buttonImageOnly: true,
            changeMonth: true,
            changeYear: true,
            yearRange: &quot;-100:+100&quot;,
            firstDay: 1,
            onClose: function() {
                $(&quot;#candidateSearch_fromDate&quot;).trigger('blur');
            }            
        });
        
        //$(&quot;img.ui-datepicker-trigger&quot;).addClass(&quot;editable&quot;);
        
        $(&quot;#candidateSearch_fromDate&quot;).click(function(){
            daymarker.show(&quot;#candidateSearch_fromDate&quot;);
            if ($(this).val() == displayDateFormat) {
                $(this).val('');
            }
        });
    
    });

  To  

    var datepickerDateFormat = 'yy-mm-dd';
    var displayDateFormat = datepickerDateFormat.replace('yy', 'yyyy');

    $(document).ready(function(){
        
        var dateFieldValue = $.trim($(&quot;#candidateSearch_toDate&quot;).val());
        if (dateFieldValue == '') {
            $(&quot;#candidateSearch_toDate&quot;).val(displayDateFormat);
        }

        daymarker.bindElement(&quot;#candidateSearch_toDate&quot;,
        {
            showOn: &quot;both&quot;,
            dateFormat: datepickerDateFormat,
            buttonImage: &quot;/webres_5acde3dbd3adc6.90334155/themes/default/images/calendar.png&quot;,
            buttonText:&quot;&quot;,
            buttonImageOnly: true,
            changeMonth: true,
            changeYear: true,
            yearRange: &quot;-100:+100&quot;,
            firstDay: 1,
            onClose: function() {
                $(&quot;#candidateSearch_toDate&quot;).trigger('blur');
            }            
        });
        
        //$(&quot;img.ui-datepicker-trigger&quot;).addClass(&quot;editable&quot;);
        
        $(&quot;#candidateSearch_toDate&quot;).click(function(){
            daymarker.show(&quot;#candidateSearch_toDate&quot;);
            if ($(this).val() == displayDateFormat) {
                $(this).val('');
            }
        });
    
    });



Method of Application
  
All
Manual
Online



                                    </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;frmSrchCandidates&quot;)/fieldset[1]/ol[1]</value>
   </webElementProperties>
</WebElementEntity>
