import React, { FormEvent, useState } from 'react';
import { Button } from "@/components/ui/button";
import { LegalForm } from '@/models/entities/Dao';
import { BasicInfoCard } from '../cards/BasicInfoCard';
import { HorizontalActionContainer } from '@/layouts/HorizontalActionContainer';
import { LocationInfoCard } from '../cards/LocationInfoCard';
import { MembersInfoCard } from '../cards/MembersInfoCard';
import { Principal } from '@dfinity/principal';
import { DaoAssociationInitArgs } from '../../../../declarations/dao_agency/dao_agency.did.js'

type DaoFormProps = {
  onSubmit: (dao: DaoAssociationInitArgs) => Promise<Principal>;
  onCancel: () => void;
}

type FormData = {
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
  const [formData, setFormData] = useState<FormData>({
    name: '',
    address: '',
    zip: 0,
    town: '',
    legalForm: LegalForm.Association,
    purpose: '',
    boardMembers: [],
    members: []
  });

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

    const data: DaoAssociationInitArgs = {
      name: formData.name,
      address: formData.address,
      zip: formData.zip,
      town: formData.town,
      uid: `CHE-${Math.floor(Math.random() * 1000)}.${Math.floor(Math.random() * 1000)}.${Math.floor(Math.random() * 1000)}`,
      ch_id: `CH${Math.floor(Math.random() * 10000000000)}`,
      frc_id: Math.floor(Math.random() * 100000),
      purpose: formData.purpose,
      board: formData.boardMembers.map(b => Principal.fromText(b)),
      members: formData.members.map(m => Principal.fromText(m))
    };

    try {
      await onSubmit(data);

      clearForm();

      onCancel();
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-8">
      <BasicInfoCard name={formData.name} purpose={formData.purpose} legalForm={formData.legalForm} onValueChange={onValueChange} />

      <LocationInfoCard address={formData.address} town={formData.town} zip={formData.zip} onValueChange={onValueChange} />

      <MembersInfoCard id="boardMembers" title="Board members" members={formData.boardMembers} onValueChange={onValueChange} />

      <MembersInfoCard id="members" title="Members" members={formData.members} onValueChange={onValueChange} />

      <HorizontalActionContainer>
        <Button type="button" variant="outline" onClick={onCancel}>
          Cancel
        </Button>
        <Button type="submit">Create DAO</Button>
      </HorizontalActionContainer>
    </form>
  )
}