import React from 'react';
import { Principal } from '@dfinity/principal';
import { useDao } from '@/providers/DaoProvider';
import { useNavigate, useParams } from 'react-router';
import TopBar from '@/components/headers/TopBar';
import { MainContainer } from '@/layouts/MainContainer';
import { BackButton } from '@/components/buttons/BackButton';
import { VotingForm, VotingFormData } from '@/components/forms/VotingForm';
import { VotingArgs } from '../../../declarations/voting/voting.did.js';
import { VotingService } from '@/services/votingService.js';
import { ICP_CANISTER_ID } from '@/constants/icp.js';
import { useAuthentication } from '@/providers/AuthenticationProvider';
import { toast } from '@/hooks/use-toast.js';
import { Voting } from '@/models/entities/Voting';
import { DaoAssociationService } from '@/services/daoAssocationService.js';
import { Action } from '@/models/entities/Action.js';
import { VOTING_FORM } from '@/constants/placeholders.js';
import { PdfFormFieldType, PdfFormFillData, PdfService } from '@/services/pdfService.js';
import { ASSOCIATION_NOTIFICATION_LETTER } from '@/constants/pdf/association-letter.js';
import { formatDate } from '@/utils/date.js';

interface Option {
    text: string;
    action: Action | null;
}

export const CreateVotingPage = () => {
    const { identity } = useAuthentication();
    const { id } = useParams();
    const navigate = useNavigate();
    const { getDao, refreshData } = useDao();

    const dao = getDao(Principal.fromText(id!));

    const composeOptions = (formData: VotingFormData): Option[] => {
        if (formData.areCustomOptions) {
            return formData.options.map((option) => {
                const args = formData.action.args.map((arg) => {
                    // First, check for the simple case where the entire argument is the placeholder.
                    if (arg === VOTING_FORM.WINNING_OPTION) {
                        return option;
                    }

                    // Next, handle the case where the placeholder is inside a JSON string (for record parameters).
                    try {
                        const parsedArg = JSON.parse(arg);
                        if (parsedArg !== null) {
                            // It's a record. Iterate over its fields and replace the placeholder if found.
                            Object.keys(parsedArg).forEach(key => {
                                if (parsedArg[key] === VOTING_FORM.WINNING_OPTION) {
                                    parsedArg[key] = option;
                                }
                            });

                            return JSON.stringify(parsedArg);
                        }
                    } catch (e) { }

                    // If it's not the placeholder and not a JSON object containing it, return it as is.
                    return arg;
                });

                return {
                    text: option,
                    action: new Action(
                        Principal.fromText(formData.action.canisterId),
                        formData.action.method,
                        args
                    )
                }
            });
        }

        return [
            {
                text: formData.options[0],
                action: new Action(
                    Principal.fromText(formData.action.canisterId),
                    formData.action.method,
                    formData.action.args
                )
            },
            {
                text: formData.options[1],
                action: null
            }
        ];
    }

    const generatePdfLetter = async (formData: VotingFormData) => {
        const data: PdfFormFillData[] = [
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_LETTER.FIELDS.DATE,
                value: formatDate(Date.now())
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_LETTER.FIELDS.UID,
                value: dao.uid
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_LETTER.FIELDS.VOTERS,
                value: formData.votersWhitelist.slice(0, 5).join('\n')
            },
            {
                type: PdfFormFieldType.TEXT,
                name: ASSOCIATION_NOTIFICATION_LETTER.FIELDS.MESSAGE,
                value: formData.notification.message
            },
        ];

        const pdfBytes = await PdfService.fill(ASSOCIATION_NOTIFICATION_LETTER.TEMPLATE_URL, data);

        return pdfBytes;
    }

    const onSubmitVoting = async (formData: VotingFormData) => {
        const votingService = new VotingService(ICP_CANISTER_ID.VOTING, identity);

        const options = composeOptions(formData);
        let pdfLetterBytes: Uint8Array;
        if (formData.notification.email && formData.notification.message) {
            pdfLetterBytes = await generatePdfLetter(formData);
        }
        const votingArgs: VotingArgs = {
            title: formData.title,
            description: formData.description,
            options: options.map((option) => [option.text, option.action ? [option.action.toDto()] : []]),
            dao_id: dao.principal,
            end_at: BigInt(formData.endAt.getTime()),
            approval_threshold: Number(formData.approvalThreshold),
            quorum: Number(formData.quorum),
            voters_whitelist: formData.votersWhitelist,
        };

        try {
            return await votingService.createVoting(votingArgs);
        } catch (error) {
            toast({
                title: "Error!",
                description: "An error occurred while creating the voting.",
                duration: 2000,
            });

            throw error;
        }
    }

    const updateDao = async (voting: Voting) => {
        const daoService = new DaoAssociationService(dao.principal, identity);

        try {
            await daoService.addPool(voting.id);
        } catch (error) {
            toast({
                title: "Error!",
                description: "An error occurred while updating the DAO.",
                duration: 2000,
            });

            throw error;
        }
    }

    const onSubmit = async (formData: VotingFormData) => {
        const voting = await onSubmitVoting(formData);

        await updateDao(voting);

        refreshData();

        navigate(`/daos/${dao.principal}`);
    }

    return (
        <>
            <TopBar />

            <MainContainer>
                <BackButton onBack={() => navigate(-1)} />

                <div className="max-w-3xl mx-auto">
                    <div className="mb-6">
                        <h1 className="text-2xl font-bold">Create New Voting</h1>
                        <p className="text-slate-600">Set up a new voting procedure for your DAO</p>
                    </div>

                    <VotingForm dao={dao} onSubmit={onSubmit} onCancel={() => navigate(-1)} />
                </div>
            </MainContainer>
        </>
    );
};

export default CreateVotingPage;