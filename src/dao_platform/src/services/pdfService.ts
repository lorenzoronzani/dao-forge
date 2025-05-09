import { PDFDocument, PDFForm, PDFTextField } from "pdf-lib";

export enum PdfFormFieldType {
    TEXT = 'text',
    CHECKBOX = 'checkbox',
}

export type PdfFormFillData = {
    type: PdfFormFieldType;
    name: string;
    value: string | boolean;
}

export abstract class PdfService {
    private static setFieldValue(form: PDFForm, data: PdfFormFillData) {
        switch (data.type) {
            case PdfFormFieldType.TEXT:
                const field = form.getTextField(data.name);
                field.setText(data.value as string);
                break;
            case PdfFormFieldType.CHECKBOX:
                const checkbox = form.getCheckBox(data.name);
                if (data.value) {
                    checkbox.check();
                } else {
                    checkbox.uncheck();
                }
                break;
        }
    }

    static async fill(pdfTemplateUrl: string, data: PdfFormFillData[]) {
        const templateBytes = await fetch(pdfTemplateUrl).then(res => res.arrayBuffer());
        const pdfDoc = await PDFDocument.load(templateBytes);

        const form = pdfDoc.getForm();

        data.forEach(d => {
            this.setFieldValue(form, d);
        });

        return pdfDoc.save();
    }

    static async discoverFormFields(pdfTemplateUrl: string) {
        const templateBytes = await fetch(pdfTemplateUrl).then(res => res.arrayBuffer());
        const pdfDoc = await PDFDocument.load(templateBytes);

        const form = pdfDoc.getForm();
        const fields = form.getFields();

        fields.forEach(field => {
            if (field instanceof PDFTextField) {
                form.getTextField(field.getName()).setText(field.getName());
            } else {
                console.log(field.getName());
            }
        });

        return pdfDoc.save();
    }

    static async openPdf(pdfBytes: Uint8Array) {
        const blob = new Blob([pdfBytes], { type: 'application/pdf' });

        const url = URL.createObjectURL(blob);

        window.open(url, '_blank');
    }
}
