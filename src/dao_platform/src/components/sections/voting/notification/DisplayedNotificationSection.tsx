import React from 'react';
import { Notification } from '@/models/entities/Notification';
import { Button } from '@/components/ui/button';
import { PdfService } from '@/services/pdfService';
import { Mail } from 'lucide-react';

interface DisplayedNotificationSectionProps {
    notification: Notification;
}

export const DisplayedNotificationSection = ({ notification }: DisplayedNotificationSectionProps) => {

    const handleOpenDocument = async () => {
        try {
            await PdfService.openPdf(new Uint8Array(notification.pdf_bytes));
        } catch (error) {
            console.error('Error opening PDF:', error);
        }
    };

    return (
        <div className="space-y-3">
            <div className="flex items-center gap-2">
                <Mail className="h-4 w-4 text-slate-500" />
                <h4 className="font-medium">Notification Letter</h4>
            </div>

            <div className="bg-slate-50 rounded-lg p-4 space-y-3 text-sm">
                <div>
                    <span className="text-slate-500">Recipient Email:</span>
                    <div className="flex items-center justify-between mt-1">
                        <p className="font-mono text-xs break-all">{notification.email}</p>
                        <Button size="sm" variant="outline" onClick={handleOpenDocument}>
                            View Document
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    );
};
