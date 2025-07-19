import { useEffect, useState } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Mail, ChevronDown } from "lucide-react";
import { VerticalLabeledComponent } from "@/components/labels/VerticalLabeledComponent";
import { NotificationFormData } from "@/components/forms/VotingForm";

type NotificationLetterCardProps = {
    onValueChange: (field: string, value: any) => void;
    notification: NotificationFormData;
}

export const NotificationLetterCard = ({ onValueChange, notification }: NotificationLetterCardProps) => {
    const [isOpen, setIsOpen] = useState<boolean>(false);

    useEffect(() => {
        if (!isOpen) {
            onValueChange('notification', {
                email: '',
                message: ''
            });
        }
    }, [isOpen]);

    const handleChange = (field: keyof NotificationFormData, value: string) => {
        notification[field] = value;

        onValueChange('notification', notification);
    };

    return (
        <Card>
            <CardHeader onClick={() => setIsOpen(!isOpen)} className="cursor-pointer">
                <CardTitle className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                        <Mail className="h-5 w-5" />
                        Notification Letter
                    </div>
                    <ChevronDown className={`h-5 w-5 transform transition-transform ${isOpen ? 'rotate-180' : ''}`} />
                </CardTitle>
                <CardDescription>
                    Specify an email and write a message to describe the changes. This is an optional step.
                </CardDescription>
            </CardHeader>
            {isOpen && (
                <CardContent className="space-y-4">
                    <VerticalLabeledComponent label="Email" htmlFor="email">
                        <Input
                            id="email"
                            type="email"
                            placeholder="Enter email address"
                            value={notification.email}
                            required
                            onChange={(e) => handleChange('email', e.target.value)}
                        />
                    </VerticalLabeledComponent>
                    <VerticalLabeledComponent label="Message" htmlFor="message">
                        <Textarea
                            id="message"
                            placeholder="Enter your message"
                            value={notification.message}
                            required
                            onChange={(e) => handleChange('message', e.target.value)}
                        />
                    </VerticalLabeledComponent>
                </CardContent>
            )}
        </Card>
    );
};
