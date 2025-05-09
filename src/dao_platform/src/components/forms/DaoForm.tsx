import React, { FormEvent, useState } from 'react';
import { Button } from "@/components/ui/button";
import { LegalForm } from '@/models/entities/Dao';
import { BasicInfoCard } from '../cards/BasicInfoCard';
import { HorizontalActionContainer } from '@/layouts/HorizontalActionContainer';
import { LocationInfoCard } from '../cards/LocationInfoCard';
import { MembersInfoCard } from '../cards/MembersInfoCard';
import { Principal } from '@dfinity/principal';
import { Loader2 } from 'lucide-react';

type DaoFormProps = {
  onSubmit: (formData: DaoFormData) => Promise<Principal>;
  onCancel: () => void;
}

export type DaoFormData = {
  name: string;
  address: string;
  zip: number;
  town: string;
  legalForm: LegalForm;
  purpose: string;
  boardMembers: string[];
  members: string[];
}

export const DaoForm = ({ onSubmit, onCancel }: DaoFormProps) => {
  const [formData, setFormData] = useState<DaoFormData>({
    name: '',
    address: '',
    zip: 0,
    town: '',
    legalForm: LegalForm.Association,
    purpose: '',
    boardMembers: [],
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