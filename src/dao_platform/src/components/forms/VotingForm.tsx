import { FormEvent, useState } from "react";
import { Loader2 } from 'lucide-react';
import { Button } from "@/components/ui/button";
import { BasicInfoCard } from "@/components/cards/voting_form/BasicInfoCard";
import VotingOptionsCard from "@/components/cards/voting_form/VotingOptionsCard";
import ThresholdCard from "@/components/cards/voting_form/ThresholdCard";
import VotersCard from "@/components/cards/voting_form/VotersCard";
import { Principal } from "@dfinity/principal";
import { Dao } from "@/models/entities/Dao";
import { HorizontalActionContainer } from "@/layouts/HorizontalActionContainer";
import ActionSection from "../sections/voting/action/ActionSection";
import { NotificationLetterCard } from "@/components/cards/voting_form/NotificationLetterCard";

type VotingFormProps = {
    dao: Dao;
    onSubmit: (formData: VotingFormData) => Promise<void>;
    onCancel: () => void;
}

export type ActionFormData = {
    canisterId: string;
    method: string;
    args: string[];
}

export type NotificationFormData = {
    email: string;
    message: string;
};

export type VotingFormData = {
    title: string;
    description: string;
    areCustomOptions: boolean;
    options: string[];
    endAt: Date;
    approvalThreshold: number;
    quorum: number;
    votersWhitelist: Principal[];
    action: ActionFormData;
    notification: NotificationFormData;
}

// Default end at is 7 days from now
const DEFAULT_END_AT = new Date(Date.now() + 7 * 24 * 60 * 60 * 1000);
DEFAULT_END_AT.setHours(0, 0, 0, 0);

export const VotingForm = ({ dao, onSubmit, onCancel }: VotingFormProps) => {
    const [formData, setFormData] = useState<VotingFormData>({
        title: '',
        description: '',
        areCustomOptions: false,
        options: ['Accept', 'Reject'],
        endAt: DEFAULT_END_AT,
        approvalThreshold: 50,
        quorum: 25,
        votersWhitelist: [],
        action: {
            canisterId: '',
            method: '',
            args: []
        },
        notification: {
            email: "",
            message: ""
        }
    });
    const [isSubmitting, setIsSubmitting] = useState(false);

    const onValueChange = (field: string, value: unknown) => {
        if (field === 'areCustomOptions') {
            if (value) {
                setFormData({
                    ...formData,
                    areCustomOptions: value as boolean,
                    options: ['Option 1', 'Option 2']
                });
            } else {
                setFormData({
                    ...formData,
                    areCustomOptions: value as boolean,
                    options: ['Accept', 'Reject']
                });
            }
        } else {
            setFormData({
                ...formData,
                [field]: value
            });
        }
    };

    const clearForm = () => {
        setFormData({
            title: '',
            description: '',
            areCustomOptions: false,
            options: ['Accept', 'Reject'],
            endAt: DEFAULT_END_AT,
            approvalThreshold: 50,
            quorum: 25,
            votersWhitelist: [],
            action: {
                canisterId: '',
                method: '',
                args: []
            },
            notification: {
                email: "",
                message: ""
            }
        });
    };

    const handleSubmit = async (e: FormEvent) => {
        e.preventDefault();

        try {
            setIsSubmitting(true);

            await onSubmit(formData);

            clearForm();
        } catch (error) {
            console.error(error);
        } finally {
            setIsSubmitting(false);
        }
    };

    return (
        <form onSubmit={handleSubmit} className="space-y-6">
            <BasicInfoCard title={formData.title} description={formData.description} onValueChange={onValueChange} />

            <VotingOptionsCard options={formData.options} areCustomOptions={formData.areCustomOptions} onValueChange={onValueChange} />

            <ActionSection daoPrincipal={dao.principal} action={formData.action} onValueChange={onValueChange} areCustomOptions={formData.areCustomOptions} />

            <ThresholdCard endAt={formData.endAt} approvalThreshold={formData.approvalThreshold} quorum={formData.quorum} onValueChange={onValueChange} />

            <VotersCard dao={dao} onValueChange={onValueChange} />

            <NotificationLetterCard
                onValueChange={onValueChange}
                notification={formData.notification}
            />

            <HorizontalActionContainer>
                <Button type="button" variant="outline" onClick={onCancel}>
                    Cancel
                </Button>
                <Button type="submit" disabled={isSubmitting}>{isSubmitting ? (
                    <>
                        <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                        Creating...
                    </>
                ) : (
                    "Create Voting"
                )}</Button>
            </HorizontalActionContainer>
        </form>
    );
}