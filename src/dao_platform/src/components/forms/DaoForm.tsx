import React, { FormEvent, useState } from 'react';
import { Button } from "@/components/ui/button";
import { LegalForm } from '@/models/entities/Dao';
import { BasicInfoCard } from '../cards/dao_form/BasicInfoCard';
import { HorizontalActionContainer } from '@/layouts/HorizontalActionContainer';
import { LocationInfoCard } from '../cards/dao_form/LocationInfoCard';
import { MembersInfoCard } from '../cards/dao_form/MembersInfoCard';
import { Principal } from '@dfinity/principal';
import { Loader2 } from 'lucide-react';
import { PersonalInfoCard } from '../cards/dao_form/PersonalInfoCard';

type DaoFormProps = {
  onSubmit: (formData: DaoFormData) => Promise<Principal>;
  onCancel: () => void;
  userPrincipal: Principal;
}

export type DaoFormData = {
  userFirstName: string;
  userLastName: string;
  userAddress: string;
  userTown: string;
  userZip: number;
  userPhone: string;
  userEmail: string;
  name: string;
  address: string;
  zip: number;
  town: string;
  legalForm: LegalForm;
  purpose: string;
  boardMembers: string[];
  members: string[];
}

export const DaoForm = ({ onSubmit, onCancel, userPrincipal }: DaoFormProps) => {
  const [formData, setFormData] = useState<DaoFormData>({
    userFirstName: '',
    userLastName: '',
    userAddress: '',
    userTown: '',
    userZip: 0,
    userPhone: '',
    userEmail: '',
    name: '',
    address: '',
    zip: 0,
    town: '',
    legalForm: LegalForm.Association,
    purpose: '',
    boardMembers: [userPrincipal.toText()],
    members: []
  });
  const [isSubmitting, setIsSubmitting] = useState(false);

  const onValueChange = (field: string, value: unknown) => {
    setFormData({
      ...formData,
      [field]: value
    });
  };

  const clearForm = () => {
    setFormData({
      userFirstName: '',
      userLastName: '',
      userAddress: '',
      userTown: '',
      userZip: 0,
      userPhone: '',
      userEmail: '',
      name: '',
      address: '',
      zip: 0,
      town: '',
      legalForm: LegalForm.Association,
      purpose: '',
      boardMembers: [],
      members: []
    });
  };

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();

    try {
      setIsSubmitting(true);

      await onSubmit(formData);

      clearForm();

      onCancel();
    } catch (error) {
      console.error(error);
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-8">
      <PersonalInfoCard userFirstName={formData.userFirstName} userLastName={formData.userLastName} userAddress={formData.userAddress} userTown={formData.userTown} userZip={formData.userZip} userPhone={formData.userPhone} userEmail={formData.userEmail} onValueChange={onValueChange} />

      <BasicInfoCard name={formData.name} purpose={formData.purpose} legalForm={formData.legalForm} onValueChange={onValueChange} />

      <LocationInfoCard address={formData.address} town={formData.town} zip={formData.zip} onValueChange={onValueChange} />

      <MembersInfoCard id="boardMembers" title="Board members" members={formData.boardMembers} onValueChange={onValueChange} />

      <MembersInfoCard id="members" title="Members" members={formData.members} onValueChange={onValueChange} />

      <HorizontalActionContainer>
        <Button type="button" variant="outline" onClick={onCancel} disabled={isSubmitting}>
          Cancel
        </Button>
        <Button type="submit" disabled={isSubmitting}>{isSubmitting ? (
          <>
            <Loader2 className="mr-2 h-4 w-4 animate-spin" />
            Creating...
          </>
        ) : (
          "Create DAO"
        )}</Button>
      </HorizontalActionContainer>
    </form>
  )
}